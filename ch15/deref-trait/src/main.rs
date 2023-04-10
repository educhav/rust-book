fn main() {
    let mut x = 5;
    // let y = &x;
    let y = Box::new(x);
    // let y2 = &x;

    x = 7;

    println!("y is {}", y);
    println!("x is {}", x);

    assert_eq!(7, x);
    assert_eq!(5, *y);

    let x = 5;
    let y = MyBox::new(x);

    // dereferencing does not take ownership, it returns a reference
    assert_eq!(5, x);
    assert_eq!(5, *y);
    
    let m = MyBox::new(String::from("Rust"));
    // deref coercion
    // rust will automatically call deref() until we match the function signature, if possible
    // rust calls MyBox<String>.deref() which returns &String, then rust calls String.deref() which
    // returns &str which matches the function signature
    //
    // No run-time penalty, this is resolved at compile-time
    //
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>
    hello(&m);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

fn hello(name: &str) {
    println!("Hello, {name}!");
}

// will override the * operator on immutable references of MyBox<T>
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// can also implement DerefMut to override the * operator on mutable references of MyBox<T>
