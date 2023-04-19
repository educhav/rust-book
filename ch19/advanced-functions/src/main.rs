// Function pointers: a way to pass functions to other functions
// Function pointers implement all three of the closure traits
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    let list_of_numbers = vec![1, 2, 3];
    // using closure
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    // using function pointer
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();


    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// enum variants can define initializer functions
// we can use these initializer functions with any function that takes in a fp/closure
enum Status {
    Value(u32),
    Stop,
}


// can't return closures from functions since they do not have a concrete type and will have an
// unknown size at compile time
// we can box the value to return it
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
