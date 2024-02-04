struct Foo<T: ?Sized> {
    buff: T      // 大きさが異なる配列でも格納できる
}

fn main() {
    let a = [1,2,3];
    let b = [1,2,3,4,5,6];
    let c = Foo { buff: a };
    let d = Foo { buff: b };
    let e = Foo { buff: [1,2,3,4,5,6,7,8,9]};
    println!("{:?}", c.buff);
    println!("{:?}", d.buff);
    println!("{:?}", e.buff);
}
