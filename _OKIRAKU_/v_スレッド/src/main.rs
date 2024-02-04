use std::{thread, time};

fn foo(msg: &str, n: u64) {
    let m = time::Duration::from_millis(n);
    for _ in 1 .. 10 {
        println!("{}", msg);
        thread::sleep(m);
    }
}

fn main() {
    let t1 = thread::spawn(|| {
        foo("oops", 500);
    });
    let t2 = thread::spawn(|| {
        foo("piyopiyo", 400);
    });
    println!("{:?}", t1.join().unwrap());
    println!("{:?}", t2.join().unwrap());
}
