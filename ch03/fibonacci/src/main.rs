fn main() {
    println!("Hello, world!");
    println!("fib(1) = {}", fib(1));
    println!("fib(2) = {}", fib(2));
    println!("fib(3) = {}", fib(3));
    println!("fib(4) = {}", fib(4));
    println!("fib(5) = {}", fib(5));
    println!("fib(300) = {}", fib(300));
}

fn fib(n: u128) -> u128 {
    if n <= 1 {
        return 1;
    }

    fib(n-1) + fib(n-2)
}
