fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5, 'h');
    // Rust is an expression-based language
    // Statements: instructions that perform an action, do not return a value
    // Expressions: evaluate to a resultant value

    // Function definitions and assignments are statements
    // Everything else is an expression
    //
    // Expressions do not include ending semicolons
    let z = {
        let x = 3;
        x + 1
    };

    println!("The value of z is: {z}");

    println!("{}",get_five());
}


// Function signatures: must declare type of each parameter
fn another_function(x: i32) {
    println!("{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Functions can return values, must specify return type
// Functions implicitly return last expression
fn get_five() -> i32 {
    5
}

// Statements evaluate to (), when used in place of an expression
fn _plus_one(x: i32) -> i32 {
    x + 1
}
