fn main() {

    let s = String::from("hello");

    // Reference types allow other functions to use the value without taking ownership
    // &s refers to the stack data of s (pointer, size, capacity)
    // The act of creating a reference is called 'borrowing'
    // References are immutable by default
    println!("{}",calculate_length(&s));
    println!("{}",s);

    // Must use a mutable reference of a mutable variable in order to change when borrowed
    let mut s = String::from("Hello");

    change(&mut s);



    
    // Cannot have multiple mutable references to the same value
    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.


    // Also cannot have a mutable reference and an immutable reference to the same value
    // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
    //
    let nothing_reference = dangle();

}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn dangle() -> String { 

    let s = String::from("hello"); 

    s
} 

// this functions returns s and transfers ownership of s to the caller
