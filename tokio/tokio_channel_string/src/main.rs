use std::time::Duration;
use tokio::sync::mpsc;

#[tokio::main()]
async fn main() {
    let (tx1, mut rx1) = mpsc::channel(2);

    let handle1 = tokio::spawn(async move  {

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            let _ = tx1.send(val).await;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }


    });

    let handle2 = tokio::spawn(async move {
        while let Some(msg) = rx1.recv().await { // 消化
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("get {}", msg);
        }

    });

    let (_r1, _r2) = (handle1.await.unwrap(), handle2.await.unwrap());
}


