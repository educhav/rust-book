
fn main() {

    // Rust uses lifetimes to ensure no dangling references are made

    // Rust uses a borrow checker and compares the lifetimes of the reference and the subject of
    // the reference. The subject of the reference must live longer than the reference
    //
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    //

    // Correct way: now the subject lives longer than the reference
    let x = 5;
    let r = &x;

    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // This example below will not compile because string2 does not live past the end of the inner
    // scope. Rust knows this due to the return lifetime annotation a' being identical to the
    // parameter lifetime annotations

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // Lifetime annotations in struct definitions
    // We need to add lifetime annotations to struct fields that are references
    // This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

    #[derive (Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };




}

// The concrete lifetime of a' will be the smaller of the lifetimes of x and y
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Return value lifetime must be related to the parameter lifetime
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }


// This could can compile due to lifetime elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// don't need to annotate the methods here, because of the third elision rule
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// Static lifetimes: valid for entire duration of program, string literals have static lifetimes
// let s: &'static str = "I have a static lifetime.";
//
//

use std::fmt::Display;

// Generics, trait bounds, lifetimes all in one function!
// Lifetimes are a type of generic so they go along with T
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
