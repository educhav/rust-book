fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // const in rust indicates immutable
    // must have a type and be defined by a compile-time expression
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("{THREE_HOURS_IN_SECONDS}");

    // You can shadow immutable variables, but you cannot reassign without 'let'
    let x = 5;
    let x = x + 1;
    // Shadowing stops either when the scope ends or it becomes overshadowed itself
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");


    // Shadowing also allows us to change types
    let spaces = "     ";
    let spaces = spaces.len();

    println!("{spaces}");


    // However if we indicate it as mutable, we cannot change its type
    // let spaces = "     ";
    // spaces = spaces.len();
    // println!("{spaces}");
}
