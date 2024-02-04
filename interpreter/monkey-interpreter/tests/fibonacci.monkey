let phi = fn(n) {
    if (n == 0) { return 0; }
    if (n == 1) { return 1; }
    phi(n-1) + phi(n-2);
};

puts( "Fib(", 29, ") =", phi(29) );
