use std::ops::Deref;

struct Foo<T: ?Sized> {
    buff: T
}

impl<T> Deref for Foo<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.buff
    }
}

struct Bar {
    num: i32
}

impl Deref for Bar {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &self.num
    }
}

fn main() {
    let a = Foo { buff: [1,2,3] };
    let b = Foo { buff: [4,5,6,7,8,9] };
    let c = Bar { num: 123 };
    let d = Bar { num: 456 };
    println!("{:?}", *a);
    println!("{:?}", *b);
    println!("{:?}", *c);
    println!("{:?}", *d);
}
