fn main() {

    // Slices of references
    let s = String::from("hello world");

    let hello = &s[0..5];

    // Slices contain a pointer to the first element of the slice and the length
    // e.g. ptr to 6, length=5
    let world = &s[6..11];

    let s = String::from("hello");

    // These are equal
    let slice = &s[0..2];
    let slice = &s[..2];


    let len = s.len();

    // These are equal
    let slice = &s[3..len];
    let slice = &s[3..];


    // These are equal
    let slice = &s[0..len];
    let slice = &s[..];


    // Note: String slice range indices must occur at valid UTF-8 character boundaries.



    let mut s = String::from("hello world");

    // word becomes an immutable reference to a slice of s
    let word = first_word_slice(&s);

    // therefore we cannot have a mutable reference at the same time
    // s.clear(); // error!

    // we use immutable reference here again
    println!("the first word is: {}", word);


    // String literals are slices pointing to the library section of the program
    let s = "Hello, world!";

    // We can improve our first_word function by taking &str as a parameter to make it more general
    // without losing any functionality


    // More slicing
    // Array slicing
    let a = [1, 2, 3, 4, 5];

    // Array slice here has type &[i32]
    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


// Use &str to indicate string slice
fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
