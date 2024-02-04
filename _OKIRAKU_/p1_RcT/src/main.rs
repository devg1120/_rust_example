use std::rc::Rc;

#[derive(Clone)]
struct Foo {
    num: i32
}

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Drop foo {}", self.num);
    }
}

fn main() {
    let a = Rc::new(Foo { num: 123 });
    println!("{}", Rc::strong_count(&a));
    let x = Box::new(Foo { num: 456 });
    println!("{}", x.num);
    {
        let b = a.clone();
        println!("{}", Rc::strong_count(&a));
        println!("{}", Rc::strong_count(&b));
        let c = &a;
        println!("{}", Rc::strong_count(&a));
        println!("{}", Rc::strong_count(c));
        let y = x.clone();
        println!("{}", y.num);
    }
    println!("{}", Rc::strong_count(&a));
}
