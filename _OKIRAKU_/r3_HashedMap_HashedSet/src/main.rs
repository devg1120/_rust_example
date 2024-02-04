//リスト : HashMap の簡単な使用例
//リスト : HashSet の簡単な使用例
//
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
 //HashMap
    let mut ht = HashMap::new();
    ht.insert("foo", 1);
    ht.insert("bar", 2);
    ht.insert("baz", 3);
    ht.insert("oops", 4);
    println!("{:?}", ht);
    println!("{}", ht.contains_key("baz"));
    println!("{}", ht.contains_key("Baz"));
    println!("{:?}", ht.get("foo"));
    println!("{:?}", ht.get("Foo"));
    println!("{:?}", ht.remove("Oops"));
    println!("{:?}", ht.remove("oops"));
    println!("{:?}", ht.insert("foo", 100));
    println!("{:?}", ht.insert("oops", 200));
    for (k, v) in ht.iter() {
        println!("{}, {}", k, v);
    }

 //HashSet
    let a: HashSet<_> = [1,2,3,4].iter().cloned().collect();
    let b: HashSet<_> = [3,4,5,6].iter().cloned().collect();
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a.contains(&1));
    println!("{}", b.contains(&1));
    let c: HashSet<_> = a.union(&b).cloned().collect();
    println!("{:?}", c);
    let d: HashSet<_> = a.intersection(&b).collect();
    println!("{:?}", d);
    let e: HashSet<_> = a.difference(&b).collect();
    println!("{:?}", e);
    println!("{}", a.is_subset(&c));
    println!("{}", b.is_subset(&c));
    println!("{}", a.is_subset(&b));
}


