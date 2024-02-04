
pub fn function() {
    println!("mylib::function");
}
pub fn function2() {
    println!("mylib::function2 ok!!");
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
