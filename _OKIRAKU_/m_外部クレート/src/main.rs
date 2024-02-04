use num::bigint::BigInt;
use num::One;

fn fact(n: i32) -> BigInt {
    if n == 0 {
        BigInt::one()
    } else {
        n * fact(n - 1)
    }
}

fn main() {
    for n in 20 .. 31 {
        println!("{}", fact(n));
    }
}
