/*
 * Rust is a statically typed language, all types must be known to the compiler at compile-time
 */

use std::io;

fn main() {
    // Sometimes we need to provide a type annotation, in cases where many types are possible
    let guess: u32 = "42".parse().expect("Error");
    println!("{guess}");


    // Scalar data types: integers, floats, chars, booleans

    // Integers (unsigned + signed)
    // Signed numbers use 2's complement
    let _x: u32 = 14;
    let _y: i32 = -14;
    
    let _x: u8 = 14;
    let _y: i8 = -14;

    let _x: u128 = 14;
    let _y: i128 = -14;
    
    // architecture dependent integer types, 64-bit or 32-bit
    let _x: usize = 1231238314;
    let _y: isize = 14;


    // Integer literals
    // Can use underscores to partition big integers or annotate type
    let _x = 123_000_000;
    let _x = 123_000_u32;

    // Hex, octal, binary representation
    let _hex = 0xff;
    let _octal = 0o77;
    let _binary = 0b10_1000;
    let _byte = b'A';

    // Integer overflow: causes panic in debug mode, but not in release mode
    // Explicitly handle integer overflow with wrapping_, checking_, overflowing_, saturating_
    // functions in the standard library


    // Floats: f64 by default, use the IEEE-754 standard
    let _x = 2.0; // f64
    let _y: f32 = 2213.0; // f32


    // Numerical operations
    // addition
    let _sum = 5 + 10;

    // subtraction
    let _difference = 95.5 - 4.3;

    // multiplication
    let _product = 4 * 30;

    // division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1
                             //

    let _quotient = 75/82;

    // remainder
    let _remainder = 43 % 5;


    // BOOLEANS

    let _t = true;
    let _f: bool = false;


    // CHARACTERS
    // 4 bytes in size
    // Unicode scalar value
    // [U+0000, U+D7FF] [U+E000, U+10FFFF]
    let _c = 'a';
    let _c: char = 'Z';
    let _heart_eyed_cat = '😻';


    // COMPOUND TYPES
    // Tuples: have a fixed length, but can be declared as mutable
    let _t: (i32, f32, u32) = (-1,1.0,1);
    let _t: (i32, f32, String) = (-1, 1.0, String::new());
    let _t: (i32, f32) = (-1, 1.0);
    let _t: (i32, f32) = (-1, 1.0);
    let _t = (-1, 1.0, String::new());

    // Destructuring a tuple
    let (_x,_y,_z) = _t;

    // indexing a tuple
    println!("{}", _t.0);
    println!("{}", _t.1);

    let _x : i32 = _t.0;
    let _x : f32 = _t.1;
    // println!("{}", _t.2);

    // Empty tuple: unit
    // Expressions implicitly return (), if they dont return any other value
    let _x : () = ();


    // ARRAYS (fixed-sized collections on the stack)
    // Type annotation [type; size]
    let _a: [i32; 5] = [1,2,3,4,5];

    // Creates [3,3,3,3,3]
    let _a = [3; 5];
    let _first = _a[0];
    let _second = _a[4];


    // Rust will bounds check at runtime, panics if out-of-bounds
    let a = [1, 2, 3, 4, 5];


    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");



}
