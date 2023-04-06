mod back_of_house {
    // If we make a struct public, its fields will still be private by default
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // we must provide an associated function since Breakfast has a private field
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // if we make an enum public, then all variants of the enum become public
    pub enum Appetizer {
        Soup(Breakfast),
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let soup = back_of_house::Appetizer::Soup(meal);


    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// use crate::front_of_house::hosting;

// aliasing
use std::fmt::Result;
use std::io::Result as IoResult;


// re-exporting for external code
pub use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // Have to use supe
        super::hosting::add_to_waitlist();

        // This is invalid since hosting is only defined in the crate root
        // hosting::add_to_waitlist();
    }
}


// Use nested paths
// use std::cmp::Ordering;
// use std::io;
use std::{cmp::Ordering,io};

// self is collections
use std::collections::{HashMap, self};

// glob operator
use std::collections::*;
