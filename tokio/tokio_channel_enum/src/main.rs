use std::fmt;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

enum Value {
    VData(Data),
}

struct Data {
    t: String,
    s: String,
}

impl Data {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Data")
            .field("t", &self.t)
            .field("s", &self.s)
            .finish()
    }
}

#[tokio::main]
async fn main() {
    let (tx1, mut rx) = mpsc::channel(16);
    let tx2 = mpsc::Sender::clone(&tx1);

    let handle_tx1 = tokio::spawn(async move {
        loop {
            let d = Value::VData(Data {
                t: String::from("type1"),
                s: String::from("string1"),
            });
            if let Err(_) = tx1.send(d).await {
                println!("receiver dropped");
                return;
            }
            tokio::spawn(sleep(Duration::from_secs(1))).await.unwrap();
        }
    });

    let handle_tx2 = tokio::spawn(async move {
        loop {
            let d = Value::VData(Data {
                t: String::from("type2"),
                s: String::from("string2"),
            });
            if let Err(_) = tx2.send(d).await {
                println!("receiver dropped");
                return;
            }
            tokio::spawn(sleep(Duration::from_secs(2))).await.unwrap();
        }
    });

    let handle_rx = tokio::spawn(async move {
        loop {
            let res = rx.recv().await.unwrap();
            match res {
                Value::VData(d) => {
                    d.call();
                    // println!("{:?}", d.t);
                }
            }
        }
    });

    handle_tx1.await.unwrap();
    handle_tx2.await.unwrap();
    handle_rx.await.unwrap();
}


