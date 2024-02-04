
mod foo_local {
    // 公開 (foo::bar::function() でアクセスできる)
    pub mod bar {
        pub fn function() {
            println!("call foo::bar::function");
        }
    }
    // 非公開 (foo の中でのみ有効)
    mod baz {
        pub fn function() {
            println!("call foo::baz::function");
        }

    }

    // 構造体 Foo (public)
    pub struct Foo {
        // フィールド a は public, b は private
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
}

mod foo;

fn main() {
    foo_local::function();
    foo_local::bar::function();
    // foo::baz::function();   コンパイルエラー  

    let a = foo_local::Foo::new(1, 2);
    println!("{}", a.a);
    // println!("{}", a.b);    コンパイルエラー
    println!("{}", a.add());

    println!("-------------------------");
    
    foo::function();
    foo::bar::function();
    // foo::baz::function();   コンパイルエラー  

    let a = foo::Foo::new(1, 2);
    println!("{}", a.a);
    // println!("{}", a.b);    コンパイルエラー
    println!("{}", a.add());

}
