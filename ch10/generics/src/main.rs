
// generics are used in the definitions of structs, methods, functions, and enums

struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// use multiple generic type parameters 
struct Point2<T, U> {
    x: T,
    y: U,
}



// impl<T> Point<T> {
//     fn x(&self) -> &T {
//         &self.x
//     }
// }

// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

impl<X1, Y1> Point<X1, Y1> {
    // <> next to method name should match the type of the parameter
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    let will_work = Point2 { x: 1, y: String::from("Stuff")};

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    // Rust uses monomorphization at compile-time to replace generic methods/enums/structs/functions
    // with concrete instances of them. So no extra runtime cost

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);


    // let wont_work = Point { x: 5, y: 4.0 };
    // when we assign the integer value 5 to x, we let the compiler know that the generic type T will be an 
    // integer for this instance of Point<T>
    //
    // Won't work since x and y need to be the same type
}

/*
 * We read this definition as: the function largest is generic over some type T. 
 * This function has one parameter named list, which is a slice of values of type T. 
 * The largest function will return a reference to a value of the same type T.
 *
 */
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // Won't work since T needs to implement PartialOrd
        // if item > largest {
            largest = item;
        // }
    }

    largest
} 
