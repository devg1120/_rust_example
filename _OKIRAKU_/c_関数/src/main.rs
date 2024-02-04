// 階乗 (再帰)
fn fact(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}

//階乗 (パターンマッチ版)

fn fact2(n: i64) -> i64 {
    match n {
        0 => 1,
        x => x * fact2(x - 1)
    }
}

// フィボナッチ数
fn fibo(n: i64) -> i64 {
    // 末尾再帰 (最適化は保証されていないようだ)
    fn fiboiter(n: i64, a: i64, b: i64) -> i64 {
        if n == 0 {
            a
        } else {
            fiboiter(n - 1, b, a + b)
        }
    }
    fiboiter(n, 0, 1)
}

// 配列の合計値を求める
fn sum_of(func: fn(i32) -> i32, seq: &[i32]) -> i32 {
    let mut acc: i32 = 0;
    for i in 0 .. seq.len() {
        acc += func(seq[i]);
    }
    acc
}

// 探索
fn assoc(key: &str, data: &[(&str, i32)]) -> i32 {
    for &(x, v) in data {
        if x == key { return v; }
    }
    -1    
}

fn main() {
    for n in 10 .. 20 {
        println!("fact :{}", fact(n));
        println!("fact2:{}", fact2(n));
    }
    for n in 40 .. 50 {
        println!("{}", fibo(n));
    }

    // 局所関数
    fn identity(x: i32) -> i32 { x }
    fn square(x: i32) -> i32 { x * x }
    fn cube(x: i32) -> i32 { x * x * x }

    let seq: [i32; 10] = [1,2,3,4,5,6,7,8,9,10];
    println!("{}", sum_of(identity, &seq));
    println!("{}", sum_of(square, &seq));
    println!("{}", sum_of(cube, &seq));

    let data = [("foo", 10), ("bar", 20), ("baz", 30)];
    println!("{}", assoc("foo", &data));
    println!("{}", assoc("bar", &data));
    println!("{}", assoc("baz", &data));
    println!("{}", assoc("oops", &data));
}
