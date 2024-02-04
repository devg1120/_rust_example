//リスト : VecDeque の簡単な使用例

use std::collections::VecDeque;

fn main() {
    // スタック (Stack) の動作
    let mut s = VecDeque::new();
    for x in 0 .. 10 {
        s.push_back(x);
    }
    println!("{:?}", s);
    while !s.is_empty() {
        println!("{}", s.pop_back().unwrap());
    }
    // キュー (Queue) の動作
    let mut q = VecDeque::new();
    for x in 10 .. 20 {
        q.push_back(x);
    }
    println!("{:?}", q);
    while !q.is_empty() {
        println!("{}", q.pop_front().unwrap());
    }
}
