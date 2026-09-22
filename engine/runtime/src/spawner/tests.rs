use super::*;
#[cfg(unix)]
use tokio::time::Duration;

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
    let job = jobs
        .create("root", None, JobVisibility::Visible)
        .await
        .unwrap();
    let cmd = dir
        .0
        .join("missing-executable")
        .to_string_lossy()
        .to_string();
    assert!(matches!(
        SpawnerBuilder::new(&cmd, &dir.0, job.clone())
            .await
            .unwrap()
            .spawn()
            .await
            .unwrap(),
        SpawnStatus::RunError(_)
    ));
    job.cancel().cancelling().await.unwrap();
    job.cancel().cancelled::<String>(None).await.unwrap();
    jobs.destroy().await.unwrap().await.unwrap().unwrap();
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
    let job = jobs
        .create("root", None, JobVisibility::Visible)
        .await
        .unwrap();
    let process_job = job.clone();
    let cwd = dir.0.clone();
    let process = tokio::spawn(async move {
        SpawnerBuilder::new("/bin/sh wait.sh", cwd, process_job)
            .await?
            .spawn()
            .await
    });
    timeout(Duration::from_secs(5), async {
        while !dir.0.join("ready").exists() {
            sleep(Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    job.cancel().cancelling().await.unwrap();
    assert!(matches!(
        timeout(Duration::from_secs(5), process)
            .await
            .unwrap()
            .unwrap(),
        Err(E::Cancelled)
    ));
    job.cancel().cancelled::<String>(None).await.unwrap();
    jobs.destroy().await.unwrap().await.unwrap().unwrap();
    assert_eq!(
        dir.events("/bin/sh"),
        vec![
            scheme::EventTy::Started,
            scheme::EventTy::Cancelling,
            scheme::EventTy::Cancelled
        ]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancellation_transition_is_strict_and_preserves_completion() {
    let dir = TestDir::new();
    let jobs = RtJobs::new(&dir.0).unwrap();
    let parent = jobs
        .create("parent", None, JobVisibility::Hidden)
        .await
        .unwrap();
    parent.start().started(Some("parent")).await.unwrap();
    let child = parent.child("child", JobVisibility::Hidden).await.unwrap();
    child.start().started(Some("child")).await.unwrap();
    parent.cancel().cancelling().await.unwrap();
    // Deliberately repeat the transition: lifecycle mistakes must remain errors.
    assert!(matches!(
        parent.cancel().cancelling().await,
        Err(E::JobState(JobStateError::JobStateAlreadySet(
            _,
            JobState::Cancelling
        )))
    ));
    assert!(child.cancel().is_cancelled());
    assert!(matches!(
        child.child("late", JobVisibility::Hidden).await,
        Err(E::Cancelled)
    ));
    assert!(matches!(
        jobs.create("late", Some(child.identity().uuid()), JobVisibility::Hidden)
            .await,
        Err(E::Cancelled)
    ));
    // Inherited cancellation leaves the child state to its executor.
    // Actual completion can still win the race with cancellation.
    child.done().success::<String>(None).await.unwrap();
    parent.done().success::<String>(None).await.unwrap();
    jobs.destroy().await.unwrap().await.unwrap().unwrap();
    assert_eq!(
        dir.events("parent"),
        vec![
            scheme::EventTy::Started,
            scheme::EventTy::Cancelling,
            scheme::EventTy::Success
        ]
    );
    assert_eq!(
        dir.events("child"),
        vec![scheme::EventTy::Started, scheme::EventTy::Success]
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cancelled_parent_does_not_launch_a_command() {
    let dir = TestDir::new();
    std::fs::write(dir.0.join("side-effect.sh"), "touch launched\n").unwrap();
    let jobs = RtJobs::new(&dir.0).unwrap();
    let job = jobs
        .create("root", None, JobVisibility::Hidden)
        .await
        .unwrap();
    job.cancel().cancelling().await.unwrap();
    assert!(matches!(
        SpawnerBuilder::new("/bin/sh side-effect.sh", &dir.0, job.clone()).await,
        Err(E::Cancelled)
    ));
    assert!(!dir.0.join("launched").exists());
    job.cancel().cancelled::<String>(None).await.unwrap();
    jobs.destroy().await.unwrap().await.unwrap().unwrap();
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn empty_command_is_rejected_before_creating_a_job() {
    let dir = TestDir::new();
    let jobs = RtJobs::new(&dir.0).unwrap();
    let parent = jobs
        .create("parent", None, JobVisibility::Hidden)
        .await
        .unwrap();
    parent.start().started(Some("parent")).await.unwrap();
    assert!(matches!(
        SpawnerBuilder::new("   ", &dir.0, parent.clone()).await,
        Err(E::SpawnEmptyCommand)
    ));
    parent
        .done()
        .success::<String>(None)
        .await
        .expect("no unfinished process job");
    jobs.destroy().await.unwrap().await.unwrap().unwrap();
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn stdin_is_closed_before_draining_stdout_and_stderr() {
    let dir = TestDir::new();
    std::fs::write(
        dir.0.join("stdin.sh"),
        "cat
printf 'stdout\n'
printf 'stderr\n' >&2
exit 7
",
    )
    .unwrap();
    let jobs = RtJobs::new(&dir.0).unwrap();
    let parent = jobs
        .create("parent", None, JobVisibility::Hidden)
        .await
        .unwrap();
    parent.start().started(Some("parent")).await.unwrap();
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        SpawnerBuilder::new("/bin/sh stdin.sh", &dir.0, parent.clone())
            .await
            .unwrap()
            .spawn(),
    )
    .await
    .expect("child must receive EOF instead of waiting for input")
    .unwrap();
    assert_eq!(
        result,
        SpawnStatus::Failed(Some(7), vec!["stdout".into(), "stderr".into()])
    );
    assert!(!parent.cancel().is_cancelled());
    parent.done().success::<String>(None).await.unwrap();
    jobs.destroy().await.unwrap().await.unwrap().unwrap();
    assert_eq!(
        dir.events("/bin/sh"),
        vec![scheme::EventTy::Started, scheme::EventTy::Failed]
    );
}

#[cfg(unix)]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shutdown_accepts_an_already_finished_process() {
    let dir = TestDir::new();
    let jobs = RtJobs::new(&dir.0).unwrap();
    let job = jobs
        .create("process", None, JobVisibility::Hidden)
        .await
        .unwrap();
    job.start().started::<String>(None).await.unwrap();
    let mut spawner = Spawn::default().cmd("/bin/true".into()).cwd(dir.0.clone());
    assert!(matches!(
        spawner
            .spawn(
                job.cancel().cancellation_owned(),
                job.journal(),
                job.progress().await.unwrap()
            )
            .await
            .unwrap(),
        SpawnStatus::Success(_)
    ));
    spawner.shutdown().await.unwrap();
    spawner.shutdown().await.unwrap();
    job.done().success::<String>(None).await.unwrap();
    jobs.destroy().await.unwrap().await.unwrap().unwrap();
}
