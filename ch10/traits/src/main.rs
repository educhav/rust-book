// inside curly brackets of trait are the behaviors that the types that implement this trait have
// any type that implements Summary must define summarize with this exact signature

use std::fmt::Display;
use std::fmt::Debug;
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    // In order to implement a trait on a type at least either the trait or type must be local
    //
    // If you attempt to implement an external trait (Display) on an external type (Vec<T>) the
    // compiler will be confused and won't know which implementation to pick




}


// We can assign a default implementation of a trait method and then use it on a type by
// specifying an empty impl block
pub trait SummaryDefault {
    fn summarize_default(&self) -> String {
        String::from("(Read more...)")
    }
}

// Use a trait-bound parameter
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Same thing, with trait bound syntax
pub fn notify_s<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// multiple parameters, MUST USE trait bound syntax to ensure both parameters are of the same type
pub fn notify_multiple<T: Summary>(item1: &T, item2: &T) {}


// multiple trait bounds - impl trait syntax
pub fn notify_multi(item: &(impl Summary + Display)) {}

// multiple trait bounds - trait bound syntax
pub fn notify_multi_tb<T: Summary + Display>(item: &T) {}

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {1}

fn some_function2<T, U>(t: &T, u: &U) -> i32 
where 
    T: Display + Clone,
    U: Clone + Debug
{1}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Won't work, you can only use Impl trait if you are returning a single type due to how the Rust
// compiler works
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }



// Uses default implementation
impl SummaryDefault for Tweet {
}

// types that implement this trait will only have to define a method body for summarize_author
pub trait SummaryInternalMethod {
    fn summarize_author(&self) -> String;

    fn summarize_internal(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl SummaryInternalMethod for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.summarize_internal())
    }
    fn summarize_internal(&self) -> String {
        format!("(Stuff from {}...)", self.summarize_internal())
    }
}


struct Pair<T> {
    x: T,
    y: T
}

// Can use trait-bounds to conditionally implement methods for Types
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


// Can also conditionally assign traits to Types with certain traits
// Called "blanket implementations"
// impl<T: Display> ToString for T {
//     // --snip--
// }
//
// We can call ToString trait methods on types (like Integer) that implement Display
// let s = 3.to_string();
