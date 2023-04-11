fn main() {


    // A string is a wrapper over a Vec<u8>
    let s = String::new();
    let init = "Stuff";
    let s = init.to_string();

    let s = String::from("Stuff");


    // to_string() is implemented on any type that implements the Display trait

    // from and to_string() are the same, merely a matter of stylistic preference


    // Strings are UTF-8 encoded
    let hello = String::from("שָׁלוֹם");



    // Appending to a string
    let mut s = String::from("foo");
    s.push_str("bar");

    // push_str takes in a &str to prevent taking ownership
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // push takes in a single character 
    let mut s = String::from("lo");
    s.push('l');


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // string concatenation requires an owned String on the left and a borrowed String on the right
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // The reason we are able to use &s2 is because the add function can coerce a &String into a &str using deref
    // coercion that turns &s2 into &s2[..]

    // "this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result. "

    // For more complicated string combining, we use format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // format! takes references so it does not take ownership of any parameters
    let s = format!("{s1}-{s2}-{s3}");


    // Rust does not support indexing for String
    // let s1 = String::from("hello");
    // let h = s1[0];

    // String length is 4 - 4 bytes long
    let hello = String::from("Hola");


    // String length is 24 - 24 bytes long
    // Each Unicode scalar value in this String takes 2 bytes of storage
    let hello = String::from("Здравствуйте");


    // Here answer will be 208 not the Cyrillic letter Ze if it could return the first byte, which is not what the user wants
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // We need to slice to read parts of a string
    // Rust will panic if we choose a slice size of 1
    let hello = "Здравствуйте";
    let s = &hello[0..4];

    // Best way to operate on strings is to be explicit about whether you want characters or bytes
    // For individual unicode scalar values, use chars()
    for c in "Зд".chars() {
        println!("{c}");
    }

    // Getting grapheme clusters is complex, so not supplemented by standard library


}
