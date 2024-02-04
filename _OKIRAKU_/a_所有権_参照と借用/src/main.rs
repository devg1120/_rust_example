fn main() {
    println!("所有権の移動----------------------------------------");

    let a = vec![1,2,3,4,5]; // vec! はベクタを生成するマクロ
    println!("{:?}", a);     // {:?} は Debug 表示
    let b = a;               // a から b に move
    println!("{:?}", b);
    // println!("{:?}", a);  コンパイルエラー
    let c: Vec<i32>;
    {
        let d = b;           // b から d に move
        // println!("{:?}", b);  コンパイルエラー
        println!("{:?}", d);
        c = d;               // d から c に move
                             // move しないと d が廃棄されるときにベクタも廃棄される
    }
    println!("{:?}", c);

    println!("参照と借用----------------------------------------");

    let a = 123;
    let ra = &a;
    let a1 = ra * 10;
    println!("{}", ra);
    println!("{}", a1);

    let mut b = 456;
    println!("{}", b);
    {
        let rb = &mut b;      // let mut rb とするとワーニング (mut は不要)
        // let rb1 = &b;      コンパイルエラー
        // println!("{}", b); コンパイルエラー
        println!("{}", rb);
        *rb = 1000;
    }
    println!("{}", b);

}
