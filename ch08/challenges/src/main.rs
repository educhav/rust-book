use std::collections::HashMap;
use std::io;


#[derive(Debug)]
struct Stats {
    median: f32,
    mode: i32
}

fn median_mode(v: &Vec<i32>) -> Stats {
    let mut v_copy = v.clone();
    v_copy.sort();

    let median_index = v_copy.len() / 2;

    let mut median: f32 = v_copy[median_index] as f32;

    if v_copy.len() % 2 == 0 {
        median = (v_copy[median_index] + v_copy[median_index-1]) as f32 / 2.0;
    }

    let mut freqs = HashMap::new();
    let mut max_count = 0;
    let mut mode = v[0];

    for num in v {
        let count = freqs.entry(*num).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = *num;
        }
    }

    Stats {
        median, mode
    }
}

fn to_pig_latin(s: &str) -> String {
    if s.len() == 0 { return String::new() };

    let mut new_phrase = String::new();

    for word in s.split_whitespace() {
        let first = match word.chars().next() {
            Some(char) => char,
            None => panic!("Error while parsing first letter of word")
        };

        let is_vowel = match first.to_ascii_lowercase() {
            'a' => true,
            'e' => true,
            'i' => true,
            'o' => true,
            'u' => true,
            _ => false
        };

        let first_missing = &word[1..];
        let end = if is_vowel { 'h' } else { first };
        let to_add = format!("{first_missing}-{end}ay ");

        new_phrase.push_str(&to_add);
    }
    return new_phrase;
}

enum Command {
    Quit,
    Retrieve,
    Add
}

fn main() {
    let stats = median_mode(&vec![1,2,3,4,5,6,7,8,9,10]);
    let stats2 = median_mode(&vec![1,2,3,4,6,7,8,9,10]);
    let stats3 = median_mode(&vec![1,2,3,4,6,7,3,9,10]);
    println!("{:#?}", stats);
    println!("{:#?}", stats2);
    println!("{:#?}", stats3);

    let phrase1 = to_pig_latin("Sex robots are the future AYE");
    println!("{}", phrase1);


    println!("Welcome to this database or whatever the fuck");
    let mut db = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Type your command below (a to add, r to retrieve, q to quit):");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let command_type = match input.chars().next() {
            Some(a) => {
                match a {
                    'a' => Command::Add,
                    'r' => Command::Retrieve,
                    'q' => Command::Quit,
                    _ => continue
                }
            },
            None => continue
        };

        match command_type {
            Command::Quit => break,
            Command::Retrieve => {
                println!("{:#?}", &db);
            },
            Command::Add => {
                let mut delimited = input.split_whitespace();

                // discard command type
                delimited.next();

                // Problem I ran into: don't insert references (person), make 
                // strings and insert those (String::from(person))
                // The problem has to do with lifetimes, the reference will become invalid once the
                // input string is re-instantiated
                let person = match delimited.next() {
                    Some(person) => String::from(person),
                    None => continue
                };
                let department = match delimited.next() {
                    Some(dept) => String::from(dept),
                    None => continue
                };

                let vector = db.entry(department).or_insert(Vec::new());
                vector.push(person);
            }
        }

    }





}
