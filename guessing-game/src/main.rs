// io is a module
use std::io;
// Ordering is an enumeration
use std::cmp::Ordering;
// Rng is a trait
use rand::Rng;

fn main() {
    // println! is a macro that prints the arg's contents to stdout
    println!("Guess the number!");

    // 1..=100 corresponds to the closed interval [1,100]
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // creates an infinite loop
    loop {
        println!("Please input your guess:");

        // mut indicates mutable, new() is an associated function that instantiates an empty string
        let mut guess = String::new();

        // & indicates a reference type
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Shadowing, rust allows us to shadow the previous value of guess
        // colon u32 will annotate the variable's type for parse()
        // use _ to catch-all
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // match is like a switch statement, except with enumerable outcomes?
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
