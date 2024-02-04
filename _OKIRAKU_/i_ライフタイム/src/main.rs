
// 配列の探索
fn find<'a, 'b, T: PartialEq>(item: &'b T, xs: &'a [T]) -> Option<&'a T> {
    for x in xs {
        if x == item { return Some(x); }
    }
    None
}


struct Foo<'a> {
    x: &'a i32
}


struct Goo<'a> {
    x: &'a i32
}

impl <'a> Goo<'a> {
    fn goo(&self) ->&i32 { self.x }

    fn goo1(&self, y: &i32) ->&i32 {
        println!("{},{}", self.x, y);
        self.x
    }    

    fn goo2<'b>(&self, y: &'b i32) ->&'b i32 {
        println!("{},{}", self.x, y);
        y
    }    
}

fn main() {
    let xs = [1,2,3,4,5,6,7,8];
    for x in 0 .. 10 {
        match find(&x, &xs) {
            Some(v) => println!("{}", v),
            None => println!("None")
        }
    }


    let y = 123;
    let z = Goo { x: &y };
    println!("{}", z.x);

    let y = 123;
    let z = Goo { x: &y };
    println!("{}", z.goo());
    let y1 = 456;
    println!("{}", z.goo1(&y1));
    println!("{}", z.goo2(&y1));

}
