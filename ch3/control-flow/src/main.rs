fn main() {
    let number = 3;

    // if expressions
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Conditional expression MUST EVALUATE to a bool
    // Rust will not convert non-bool types to bool
    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Since if is an expression, we can use it in an assignment
    // If and else must have matching types since variables need to have single type defined at
    // compile time
    let condition = true;
    // Blocks of code evaluate to the last expression in them
    let _number = if condition { 5 } else { 6 };

    
    let mut counter = 0;

    // Break and return are considered statements in rust
    // You can use break statements to return values from loops
    let _result = loop {
        counter += 1;

        if counter == 10 {
            break 2 * counter;
        }
    };


    // Loop labels: can specify which loop in the break/continue statement by its label
    // By default break/continue refers to the innermost loop where it is used
    let mut count = 0;
    
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    println!("LIFTOFF!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Slow since we have to check condition on each iteration
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }

    // Range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
