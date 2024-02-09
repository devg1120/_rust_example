/// Brute force factorial function
fn factorial(x) {
    if x == 1 { return 1; }
    x * factorial(x - 1)
}
fn x40(x) {

   print(`script func: ${x}`);
   x * 40

}

// Calling an external function 'compute'
//compute(factorial(9))
compute(x40(3))


