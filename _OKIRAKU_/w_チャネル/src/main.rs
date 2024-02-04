use std::thread;
use std::sync::mpsc;

fn fibo(n: i64) -> i64 {
    if n < 2 {
        n
    } else {
        fibo(n - 1) + fibo(n - 2)
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    for x in vec![40, 39, 38, 37] {
        let tx = tx.clone();
        thread::spawn(move || tx.send(fibo(x)).unwrap());
    }
    for _ in 0 .. 4 {
        println!("{}", rx.recv().unwrap());
    }
}
