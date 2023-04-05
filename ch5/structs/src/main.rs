
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs, behavior similarly to (), 
struct AlwaysEqual;

fn main() {

    let subject = AlwaysEqual;

    // Entire instance is mutable if we specify mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("stuff@yahoo.com"),
        sign_in_count: 1
    };
    user1.email = String::from("changed@yahoo.com");

    // struct update syntax
    // create new instance of struct with existing values of other structs
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Color(r,g,b) = black;

    println!("{}",black.0);
    println!("{}",black.0);

    // cannot borrow moved values
    // String implements Copy trait
    // username is moved into user2, so user1.email cannot be used

    println!("{}",user1.active);
    println!("{}",user1.sign_in_count);
    println!("{}",user1.email);


}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// can use field init shorthand if the local variables name is identical to the field name
fn build_user_better(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
