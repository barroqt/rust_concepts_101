mod front_of_house; // Using a semicolon after mod front_of_house rather than using a block tells Rust to load the contents of the module from another file with the same name as the module

pub use crate::front_of_house::hosting; // Brings path into scope (like an "include"). Can be called with "hosting" from that point onwards

fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // like cd .. syntax in a filesystem, goes to parent module
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String, // the customer knows and decides
        seasonal_fruit: String, // the customer does not know, the chef decides
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

use std::collections::HashMap;
// use std::io;
// use std::io::Write;
// would look like:
// use std::io::{self, Write};

fn main() {
    let mut map = HashMap::new();
    map.insert(1,2);
}
