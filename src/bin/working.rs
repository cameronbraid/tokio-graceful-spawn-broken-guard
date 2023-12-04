#![allow(clippy::print_stdout)]
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    let shutdown = tokio_graceful::Shutdown::new(tokio::time::sleep(Duration::from_millis(10)));

    // my app will spawn tasks that wish to block shutdown
    // when none of these tasks are running, shutdown should proceed normally

    // create a handle to be able to create more guards on demand, which can be cloned and passed around my app to task spawners
    let guard = shutdown.guard();

    guard.spawn_task(async move {
        // create a strong guard to block shutdown
        println!("Doing work");
        tokio::time::sleep(Duration::from_secs(2)).await;
        println!("Done working");
    });

    // drop my guard so that I don't block shutdown
    guard.downgrade();

    println!(
        "{:?}",
        shutdown.shutdown_with_limit(Duration::from_secs(5)).await
    );
}
