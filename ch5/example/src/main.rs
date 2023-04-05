#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // method: like instance method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // associated function: like static method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: 30,
        height: dbg!(50 * scale)
    };
    println!("rect1 is {:#?}",rect1);

    dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let rect2 = Rectangle::square(5);

    dbg!(&rect2);



    // Rust has automatic referencing and deferencing
    // It will match the type of rect1 (&, &mut, *) with the signature of area
    println!("{}",rect1.area());

}


// Accessing Copy trait fields of a borrowed struct does not move them
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
