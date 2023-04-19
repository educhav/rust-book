// newtype pattern is useful for creating a lightweight encapsulation mechanism
fn main() {
    // type alias
    // mostly used for long type names
    // Kilometers is not a separate type, it will be treated like a u32
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    // no type checking benefits
    println!("x + y = {}", x + y);


    // Dynamically sized types
    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    // Fix by using &str
    // General solution to allow DSTs on the stack is to use a pointer to hold the value of the DST



}

// never type: return type that signals this function doesn't return
// diverging function: a function that never returns
fn bar() -> ! {
    // --snip--
    loop {}
}


// by default generic functions will only work with types that implement Sized: whose sizes are
// known at compile time.
// You can get around this restriction using ?Sized in the type annotation
// uh.. why would you ever need this? 
fn generic<T: ?Sized>(t: &T) {
    // --snip--
}
