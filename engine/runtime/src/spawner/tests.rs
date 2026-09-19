use super::*;

struct TestDir(PathBuf);
impl TestDir {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!("sibs-spawner-{}", Uuid::new_v4()));
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn events(&self, alias: &str) -> Vec<scheme::EventTy> {
        let mut reader = JournalReader::new(&self.0).unwrap();
        let sessions = reader.list();
        let session = sessions.keys().next().unwrap();
        let count = reader.open(session).unwrap().unwrap();
        let records = reader.read(session, 0, count).unwrap();
        let id = records
            .iter()
            .find(|r| r.event == scheme::EventTy::Started && r.msg == alias)
            .unwrap()
            .uuid;
        records
            .into_iter()
            .filter(|r| r.uuid == id && r.event != scheme::EventTy::Log)
            .map(|r| r.event)
            .collect()
    }
}
impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn setup_failure_finishes_the_job() {
    let dir = TestDir::new();
    let jobs = RtJobs::new(&dir.0).unwrap();
    let job = jobs.create("root", None).await.unwrap();
    let cmd = dir
        .0
        .join("missing-executable")
        .to_string_lossy()
        .to_string();
    assert!(matches!(
        spawn(&cmd, &dir.0, job).await.unwrap(),
        SpawnStatus::RunError(_)
    ));
    jobs.destroy().await.unwrap();
    assert_eq!(
        dir.events(&cmd),
        vec![scheme::EventTy::Started, scheme::EventTy::Failed]
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn inherited_cancellation_finishes_the_process_job() {
    use tokio::time::{sleep, timeout, Duration};
    let dir = TestDir::new();
    std::fs::write(
        dir.0.join("wait.sh"),
        "printf ready > ready\nexec sleep 60\n",
    )
    .unwrap();
    let jobs = RtJobs::new(&dir.0).unwrap();
    let job = jobs.create("root", None).await.unwrap();
    let process_job = job.clone();
    let cwd = dir.0.clone();
    let process = tokio::spawn(async move { spawn("/bin/sh wait.sh", cwd, process_job).await });
    timeout(Duration::from_secs(5), async {
        while !dir.0.join("ready").exists() {
            sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    job.cancel().cancel().await.unwrap();
    assert_eq!(
        timeout(Duration::from_secs(5), process)
            .await
            .unwrap()
            .unwrap()
            .unwrap(),
        SpawnStatus::Cancelled
    );
    jobs.destroy().await.unwrap();
    assert_eq!(
        dir.events("/bin/sh wait.sh"),
        vec![
            scheme::EventTy::Started,
            scheme::EventTy::Cancelling,
            scheme::EventTy::Cancelled
        ]
    );
}
