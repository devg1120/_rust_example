//リスト : モジュール foo/mod.rs

pub mod bar;
mod baz;

pub struct Foo {
    pub a: i32, b: i32
}

impl Foo {
    pub fn new(a: i32, b: i32) -> Foo {
        Foo { a: a, b: b }
    }
    pub fn add(&self) -> i32 { self.a + self.b }
}

pub fn function() {
    println!("call foo::function");
    baz::function();
}

