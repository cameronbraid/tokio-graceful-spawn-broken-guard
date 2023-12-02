use std::sync::Arc;

use tokio::sync::Mutex;

pub struct GuardState {
  guard: Mutex<Option<tokio_graceful::ShutdownGuard>>,
  weak: tokio_graceful::WeakShutdownGuard,
}

impl GuardState {

  pub fn arc(weak: tokio_graceful::WeakShutdownGuard) -> Arc<Self> {
    Arc::new(Self {
      guard: Mutex::new(None),
      weak,
    })
  }
  pub async fn upgrade(&self) {
    self
    .guard
    .lock()
    .await
    .replace(self.weak.clone().upgrade());
  }
}