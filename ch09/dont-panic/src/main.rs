use std::fs::{self,File};
use std::io::{self, Read, ErrorKind};
use std::error::Error;

// Box<dyn Error> means any kind of error
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file_result = File::open("hello.txt");

    // File::open returns io::Error struct inside of the Err variant
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other => {
                panic!("Problem opening the file: {:?}", other);
            }
        }
    };


    


    // The ? operator can only be used in functions whose return type is compatible with the value the ? is used on.
    // let greeting_file = File::open("stff")?;

    // unwrap will return the value inside of the Ok variant, or panic with a printout of the Error
    // expect is the same, but lets you enter in a custom panic message as well
    // let greeting_file = File::open("hello2.txt").unwrap();
    // let greeting_file = File::open("hello2.txt").expect("Custom panic message");
    //

    // Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? 
    // operator on an Option in a function that returns Option, but you can’t mix and match.
    
    // You can change main to return a Result<T, E>
    // When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) 
    // and will exit with a nonzero value if main returns an Err value.
    let greeting_file = File::open("stff")?;
    Ok(())

    // The main function may return any types that implement the std::process::Termination trait, 
    // which contains a function report that returns an ExitCode. 
}

// Will propagate Err to caller function
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}


// equivalent, using ?
// ? will call from on the Error returned from ? to convert to the Error returned by the function
fn read_username_from_file_better() -> Result<String, io::Error> {
    // ? will either return the value inside Ok, or return from the whole function with the value in Err
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_better2() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_best() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// can use ? with functions that return Option as well
// Same behavior, will return early with None, otherwise unwrap value
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}



pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
