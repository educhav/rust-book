fn main() {
    // explicit type annotation
    let v: Vec<i32> = Vec::new();

    // vec! is a macro that creates a new vector for us
    let v = vec!([1,2,3]);


    
    // note: rust compiler can infer type from the push operation as well
    let mut v = Vec::new();
    v.push(5);


    let v = vec![1, 2, 3, 4, 5];

    let third: i32 = v[2];
    let third: &i32 = &v[2];

    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    
    let v = vec![1, 2, 3, 4, 5];

    // will panic
    let does_not_exist = &v[100];
    // will return None
    let does_not_exist = v.get(100);



    // Rules of borrowing apply here: cannot have a mutable and immutable borrow in the same scope
    // Applies for vector as a whole due to the vector potentially moving on the heap
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
    //

    // Iterating through a vector: immutable references
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    // the reference to the vector that the for loop holds prevents simulataneous changes to the
    // vector in the body
    for i in &mut v {
        *i += 50;
    }
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];


    // Vectors are dropped
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
}
