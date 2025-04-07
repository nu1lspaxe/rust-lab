mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast,
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use crate::back_of_house::Breakfast;

// use crate::back_of_house::{
//     self as other_back_of_house,
//     Breakfast,
//     Appetizer,
// };

use crate::back_of_house::*;

pub fn eat_at_restaurant() {
    let mut meal = Breakfast::summer(String::from("Rye"));
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    fix_incorrect_order();
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}