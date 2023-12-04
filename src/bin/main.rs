#![allow(clippy::print_stdout)]
use std::time::Duration;

#[tokio::main(flavor = "multi_thread")]
pub async fn main() {
    let shutdown = tokio_graceful::Shutdown::new(tokio::time::sleep(Duration::from_millis(500)));

    tokio::spawn({
        let weak = shutdown.guard_weak();
        async move {
            let _strong = weak.upgrade();
            println!("Doing work");
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("Done working");
        }
    });

    println!(
        "{:?}",
        shutdown.shutdown_with_limit(Duration::from_secs(1)).await
    );
}
