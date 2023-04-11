use std::collections::HashMap;


fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    let team_name = String::from("Blue");

    // scores.get(&team_name) will return Option<&i32>: None() if key was not found
    // copied() will convert Option<&i32> to Option<i32>
    // unwrap_or will set score to the value in Some() or 0 if None() is returned
    let score = scores.get(&team_name).copied().unwrap_or(0);


    // Iterate through HashMap with immutable reference
    for (key, value) in &scores {
        println!("{key}: {value}");
    }


    // HashMaps take ownership if non-Copy type
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // field_name and field_value are invalid
    //

    // If we insert references into a HashMap, their values are not moved into the HashMap, but
    // the references must be valid while the HashMap is valid


    // Updating a HashMap:
    // 1. Overwriting a value

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    // "Blue" will map to 25 after this

    println!("{:?}", scores);

    // 2. Adding a key/value pair only if the key does not exist
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // entry() returns an enum Entry that represents a value that might or might not exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    // or_insert() returns a mutable reference to the value inserted or kept

    println!("{:?}", scores);


    // 3. Updating the value based off of the previous value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    // split_whitespace() returns an iterator over sub-slices separated by whitespace of the value text

    println!("{:?}", map);

}
