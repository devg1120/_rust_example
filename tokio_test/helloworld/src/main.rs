use tokio::time::Duration;

#[tokio::main] // 忘れないで！
async fn main() {
    tokio::task::spawn(async {  // タスクを記述する
        tokio::time::sleep(Duration::from_secs(3)).await; // ネットワーク呼び出しのフリ
        println!("woke up!"); // 3秒後に実行されるよ
    });
    // この時点でタスクはすでに走り始めている
    
    std::thread::sleep(Duration::from_secs(5)); // プログラムが終了しないように
    println!("Done");
}

