#![allow(clippy::print_stdout)]
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    let shutdown = tokio_graceful::Shutdown::new(tokio::time::sleep(Duration::from_millis(500)));

    // my app will spawn tasks that wish to block shutdown
    // when none of these tasks are running, shutdown should proceed normally

    // create a handle to be able to create strong guards on demand, which can be cloned and passwd around my app
    let weak = shutdown.guard_weak();

    // simulate my app spawning a task that blocks shutdown
    tokio::spawn(async move {

        // create a strong guard to block shutdown
        let _strong = weak.clone().upgrade();
        println!("Doing work");
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("Done working");
    });

    println!(
        "{:?}",
        shutdown.shutdown_with_limit(Duration::from_secs(1)).await
    );
}
