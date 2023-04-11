/*
 *  Ownership in rust: method to heap memory management
 *  1. Every value in Rust has an owner
 *  2. There can only be one owner at a time
 *  3. When the owner goes out of scope, the value is dropped
 */
fn main() {
    // Ownership
    // Scope is the range within a program for which an item is valid
    {
        let _s = "hello";
    }
    // Once _s leaves the scope where it was declared, it becomes invalid

    // String literals are immutable, Strings are mutable and stored on the heap
    // Create a String from a string literal
    {
        let mut _s = String::from("hello");
        _s.push_str(" world!");
    }
    // once _s is out of scope, String::drop() is implicitly called and the heap memory associated
    // with _s is returned to the allocator
    //

    // Works as expected
    let _x = 5;
    let _y = _x;

    // When _s2 is assigned to _s1, _s1 is invalidated to prevent double free error
    // We say that _s1 is moved into _s2 here, rather than _s2 being a shallow copy
    let _s1 = String::from("stuff");
    let _s2 = _s1;

    // Rust will never automatically create deep copies of variables


    // Make deep copy by using common method clone()
    let _s1 = String::from("stuff");
    let _s2 = _s1.clone();

    println!("{}", _s1);
    println!("{}", _s2);

    // Do not need to use clone() for stack-only data, rust will make a copy of stack data since
    // its size is known at compile-time
    // so _x is still valid after copying to _y
    let _x = 5;
    let _y = _x;

    // Types can use the annotation Copy to have it trivially copied on the stack rather than moved
    // Can't use Copy trait with a type that has the Drop trait implemented

    let s = String::from("hello");

    takes_ownership(s);
    // String does not have Copy trait, so s becomes invalid afterwards

    // println!("{}", s);

    let x = 5;
    // i32 has Copy trait, so x is still valid afterwards
    makes_copy(x);

    let _s1 = gives_ownership();

    let s2 = String::from("hello");

    // Return can transfer ownership as well
    // s2 is given to the function being called, then given back to s3
    let _s3 = takes_and_gives_back(s2);


    // at the end of this block, s1 and s3 are dropped, and s2 was already moved by the function
    // call


    // Return can return multiple values through a tuple
    //
    let s = String::from("hello");

    let (s2, len) = calculate_length(s);

    println!("{} {}", s2, len);
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope
    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
