use std::time::Duration;
use num_cpus;

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    let cpus = num_cpus::get();
    println!("logical cores: {}", cpus);

    tokio::task::spawn(async move {
        println!("task 1 started...");
        std::thread::sleep(Duration::from_secs(3)); // 3秒寝る(同期スリープ)
        println!("task 1 woke up!");
    });
    // この時点でtask1はすでに走っている。
    
    tokio::task::spawn(async move {
        println!("task 2 started...");
        std::thread::sleep(Duration::from_secs(1));
        println!("task 2 woke up!");
    });
    // この時点でtask2は走らない。空きスレッドがないから。
    std::thread::sleep(Duration::from_secs(5)); // 3秒後にtask2は走る

    println!("Done");
}
