/// @Author: Mitul
/// @Date:   2024-11-15 20:05:04
/// @Description: Library Crate
/*
- By using modules, programmer can group related definitions together and name why they’re related. Programmers using this code can navigate the code based on the
  groups rather than having to read through all the definitions, making it easier to find the definitions relevant to them. Programmers adding new functionality to
  this code would know where to place the code to keep the program organized.
- Modules can contain other modules inside of them, strcuts, enums, constants and traits.
- crate
    └── front_of_house
        ├── hosting
        │   ├── add_to_waitlist
        │   └── seat_at_table
        └── serving
            ├── take_order
            ├── serve_order
            └── take_payment
- To reference an item a module tree, the path to that item is need to be specified. Paths are declared by identifiers separated by (::).
- By default a child module and everything inside it is private w.r.t to parent but a child module is able to access anything defined in the parent module.
- Relative paths can be constructed that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path. This
  is like starting a filesystem path with the .. syntax. Using super allows us to reference an item that we know is in the parent module, which can make rearranging
  the module tree easier when the module is closely related to the parent but the parent might be moved elsewhere in the module tree someday.
*/
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod module_different_file; //Rust finds this module in another files whose name matches the module name.

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
fn deliver_order() {} // Defined in the top crate module.

mod back_of_house_super {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
pub fn eat_at_restaurant_path() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
    // For the same reason, a strcture of above type will be constructed
    // using the summer function and not directly.
}

mod back_of_house_enum {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_enum() {
    let order1 = back_of_house_enum::Appetizer::Soup;
    let order2 = back_of_house_enum::Appetizer::Salad;
}

pub use crate::front_of_house::hosting; // Brings the module into scope
pub fn eat_at_restaurant_use_keyword() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--use std::io::Result as IoResult;
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function3() -> Result {
//     // --snip--
// }

// fn function4() -> IoResult<()> {
//     // --snip--
// }
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
use rand::Rng;
use rand::{CryptoRng, ErrorKind::Transient}; // Nested Paths
fn rand_num() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}

use std::collections::*;
use std::io::{self, Write};
// use std::{cmp::Ordering, io};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
