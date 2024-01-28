#![allow(dead_code)]

// test struct //////////////////////////////////////////////////////

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
// ユニット
struct Unit;

// A tuple struct
// タプル
struct Pair(i32, f32);

// A struct with two fields
// 2つのフィールドを持つ（クラシックな）構造体
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
// 構造体は他の構造体のフィールドになることができる
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    // 長方形は座標空間上における左上隅と右下隅の位置によって指定できる
    top_left: Point,
    bottom_right: Point,
}

fn test_struct() {

    // Create struct with field init shorthand
    // 構造体をフィールド初期化の簡略記法で生成
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    // 構造体のデバッグ表示を行う
    println!("{:?}", peter);


    // Instantiate a `Point`
    // `Point` のインスタンス化
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    // pointのフィールドにアクセスする。
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    // 構造体の更新記法を用いて、別の構造体のフィールドの値を基に
    // 新たなpointを生成
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    // `bottom_right.y` の値は `point.y` と同一になるが、
    // これは `point` のフィールドの値を用いて生成したためである
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    // `let`を使用してpointをデストラクトする。
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        // 構造体の定義とインスタンスの作成を同時に行う
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    // ユニットをインスタンス化
    let _unit = Unit;

    // Instantiate a tuple struct
    // タプルをインスタンス化
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    // タプルのフィールドにアクセス
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    // タプルをデストラクト
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

// https://doc.rust-jp.rs/rust-by-example-ja/custom_types/enum.html
// test enum //////////////////////////////////////////////////////

enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    // `enum`要素型はユニット風でもよい
    PageLoad,
    PageUnload,
    // like tuple structs,
    // タプル風でもよい
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    // C言語スタイルの構造体風でもよい
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
// 引数として`WebEvent`列挙型をとり、何も返さない関数
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn test_enum() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    // `to_owned()`は文字列スライスから所有権のある`String`を作成する
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

// test struct //////////////////////////////////////////////////////


fn main() {

   println!("test struct ------------------------------");
   test_struct();
   println!("test enum ------------------------------");
   test_enum();

}
