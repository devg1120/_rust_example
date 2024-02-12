use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main()]
async fn main() {
    // チャネル (胃)
    let (tx1, mut rx1) = mpsc::channel(2);

    // 送信側タスク (口)
    let handle1 = tokio::spawn(async move {
        let mut count: i32 = 0;

        while count < 5 {
            if let Err(e) = tx1.send(count).await { // 食べる
                eprintln!("{}", e);
                break;
            }
            println!("sent {}", count);
            count += 1;
        }
    });
    // この時点で食事はすでに進んでいます。

    // 受信側タスク (小腸)
    let handle2 = tokio::spawn(async move {
        while let Some(msg) = rx1.recv().await { // 消化
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("get {}", msg);
        }
    });

    let (_r1, _r2) = (handle1.await.unwrap(), handle2.await.unwrap());
}


