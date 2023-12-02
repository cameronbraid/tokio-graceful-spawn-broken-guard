#![allow(clippy::print_stdout)]
use std::time::Duration;

use tokio_graceful_spawn_broken_guard::GuardState;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    let shutdown = tokio_graceful::Shutdown::new(tokio::time::sleep(Duration::from_millis(500)));

    let state = GuardState::arc(shutdown.guard_weak());

    tokio::spawn({
        let state = state.clone();
        async move {
            state.upgrade().await;
            println!("guard is set");
        }
    });

    println!(
        "{:?}",
        shutdown.shutdown_with_limit(Duration::from_secs(1)).await
    );
}
