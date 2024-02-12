
use std::time::Duration;
use futures::{stream::FuturesUnordered, StreamExt};

// #[tokio::main]
#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() {
    console_subscriber::init();

    let mut tasks = FuturesUnordered::new();

    let h1 = tokio::spawn(async move {
        std::thread::sleep(Duration::from_secs(10)); // ここでつまるよ！
        println!("0 woke up!");
    });
    tasks.push(h1);
    println!("0 started...");

    for i in 1..1000 {
        let h2 = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("{i} woke up!");
        });
        tasks.push(h2);
        println!("{i} ...");
    }

    // join_allは遅いよ。https://github.com/tokio-rs/tokio/issues/2401
    while let Some(item) = tasks.next().await {
        let () = item.unwrap();
    }
    println!("Done");
}
