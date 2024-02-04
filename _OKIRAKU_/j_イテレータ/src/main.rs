// start 以上 end 未満の整数列を生成する
struct IntSeq {
    start: i32, end: i32
}

// 初期化
impl IntSeq {
    fn new(s: i32, e: i32) -> IntSeq {
        IntSeq { start: s, end: e }
    }
}

// イテレータの実装
impl Iterator for IntSeq {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.start < self.end {
            let x = self.start;
            self.start += 1;
            Some(x)
        } else {
            None
        }
    }
}

fn main() {
    let mut s0 = IntSeq::new(0, 5);
    while let Some(x) = s0.next() {
        println!("{}", x);
    }
    for x in IntSeq::new(5, 10) {
        println!("{}", x);
    }
    let mut s1 = 10 .. 15;
    while let Some(x) = s1.next() {
        println!("{}", x);
    }

    //  配列と for 文 
    let a = [1, 2, 3, 4, 5];
    for x in a.iter() {
        println!("{}", x);
    }
}

