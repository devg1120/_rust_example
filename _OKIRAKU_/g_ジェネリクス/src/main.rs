// 配列からデータを探す
fn find_if<T>(pred: fn(&T) -> bool, table: &[T]) -> Option<&T> {
    for x in table {
        if pred(x) { return Some(x); }
    }
    None
}

// 組
#[derive(Debug, PartialEq)]
struct Pair<T, U> {
    fst: T, snd: U
}

impl <T, U> Pair<T, U> {
    fn new(a: T, b: U) -> Pair<T, U> {
        Pair { fst: a, snd: b }
    }
}


//
// 配列の探索
fn find<T: PartialEq + Copy>(key: T, data: &[T]) -> bool {
    for &x in data {
        if key == x { return true; }
    }
    false
}

// 連想リストの探索
fn assoc<T: PartialEq + Copy, U>(key: T, data: &[(T, U)]) -> Option<&U> {
    for &(x, ref v) in data {
        if key == x { return Some(v); }
    }
    None
}
fn main() {
    fn evenp(x: &i32) -> bool { x % 2 == 0 }
    fn oddp(x: &i32) -> bool { x % 2 != 0 }
    let a = [2, 4, 6, 8, 10];
    match find_if(evenp, &a) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    match find_if(oddp, &a) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }


    //----------------------
     let p1 = Pair::new(1, 2.0);
    let p2 = Pair::new(1, 3.0);
    let p3 = Pair::new("foo", 100);
    println!("{:?}", p1);
    println!("{:?}", p2);
    println!("{:?}", p3);
    println!("{}", p1 == p2);
    println!("{}", p3 == p3);
    //----------------------
    let data = [1,2,3,4,5,6,7,8];
    println!("{}", find(8, &data));
    println!("{}", find(0, &data));

    let data1 = ["foo", "bar", "baz"];
    println!("{}", find("baz", &data1));
    println!("{}", find("oops", &data1));

    let data2 = [("foo", 100), ("bar", 200), ("baz", 300)];
    match assoc("baz", &data2) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
    match assoc("oops", &data2) {
        Some(v) => println!("{}", v),
        None => println!("None")
    }
}
