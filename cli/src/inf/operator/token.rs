use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tokio::sync::Notify;
use tokio_util::sync::CancellationToken;

#[derive(Clone, Debug)]
pub struct OperatorToken {
    pub signal: CancellationToken,
    pub confirmation: CancellationToken,
    state: Arc<AtomicBool>,
    childs: Vec<OperatorToken>,
}

impl OperatorToken {
    pub fn new() -> Self {
        Self {
            signal: CancellationToken::new(),
            confirmation: CancellationToken::new(),
            state: Arc::new(AtomicBool::new(false)),
            childs: Vec::new(),
        }
    }

    pub fn get_confirmation(&self) -> Box<dyn FnOnce() + Send> {
        let state = self.state.clone();
        let confirmation = self.confirmation.clone();
        Box::new(move || {
            state.store(true, Ordering::SeqCst);
            confirmation.cancel();
        })
    }
    pub fn cancel(&self) {
        println!(
            ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> CHILDS TO CANCEL: {}",
            self.childs.len()
        );
        if self.state.load(Ordering::Relaxed) {
            return;
        }
        self.childs.iter().for_each(|child| {
            child.cancel();
        });
        self.signal.cancel();
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> CANCEL SIGNAL SENT",);
    }

    pub async fn cancelled(&self) {
        self.signal.cancelled().await
    }

    pub async fn finished(&self) {
        if self.state.load(Ordering::Relaxed) || self.confirmation.is_cancelled() {
            return;
        }
        Box::pin(async move {
            for child in self.childs.iter() {
                child.finished().await;
            }
        })
        .await;
        self.confirmation.cancelled().await;
    }

    pub async fn childs_finished(&self) {
        if self.state.load(Ordering::Relaxed) {
            return;
        }
        println!(
            ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> WAITING FOR {} CHILDS TO BE DONE",
            self.childs.len()
        );
        for child in self.childs.iter() {
            child.finished().await;
        }
        println!(
            ">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> {} CHILDS ARE DONE",
            self.childs.len()
        );
    }

    pub fn finish(&self) {
        if self.state.load(Ordering::Relaxed) {
            return;
        }
        println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>> FINISH IS CALLED");
        for child in self.childs.iter() {
            child.finish();
        }
        self.state.store(true, Ordering::SeqCst);
        self.confirmation.cancel();
    }

    pub fn child(&mut self) -> OperatorToken {
        let child = OperatorToken::new();
        self.childs.push(OperatorToken {
            signal: child.signal.clone(),
            confirmation: child.confirmation.clone(),
            state: child.state.clone(),
            childs: Vec::new(),
        });
        child
    }
}
