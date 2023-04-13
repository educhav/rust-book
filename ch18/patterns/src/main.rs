fn main() {
    println!("Hello, world!");

    // PATTERNS IN WHILE LOOPS
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }


    // PATTERNS IN IF-LET expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // PATTERNS FOR DESTRUCTURING TUPLES IN FOR LOOPS
    let v = vec!['a', 'b', 'c'];

    // (index, value) is the pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }


    // PATTERNS FOR VARIABLE ASSIGNMENT
    // 'x' is a pattern that means bind the value to 'x'
    let x = 5;

    let (x,y,z) = (1,2,3);
    // let (x,y,_) = (1,2,3);
    let point = (3, 5);
    print_coordinates(&point);

    let some_option_value: Option<&str> = None;
    // cannot use refutable patterns in local bindings
    // use if let instead so the code can have a valid way to continue if None
    // let Some(x) = some_option_value;


    // Irrefutable pattern in if-let is pointless
    if let x = 5 {
        println!("{}", x);
    };
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

