fn main() {

    println!("配列とスライス------------------------------");

   let a = [1,2,3,4,5,6,7,8];
    println!("{}", a[0]);
    println!("{}", a.len());
    let a1 = a;               // 基本的なデータ型は代入演算子でコピーされる
    println!("{:?}", a);
    println!("{:?}", a1);

    let mut b = [0; 5];
    b[0] = 10;
    println!("{}", b[0]);
    let mut b1 = b;           // mutable な配列もコピーされる
    b1[0] = 20;
    println!("{:?}", b);
    println!("{:?}", b1);
    
    let c = &a[2..5];         // immutable のスライス (参照) はいくつでも作れる
                              // 配列自体はコピーされない
    println!("{:?}", c);
    println!("{}", c.len());

    let d = &a[3..];
    println!("{:?}", d);
    println!("{}", d.len());

    {
        let e = &mut b[..];   // mutable な参照はひとつだけ
                              // let mut e はワーニング (mut は不要)
        e[1] = 20;
        println!("{}", e[1]);
        // b[1] = 30;   コンパイルエラー
    }
    b[1] = 30;                   // e が破棄されたので b にアクセスできる
    println!("{}", b[1]);
}
