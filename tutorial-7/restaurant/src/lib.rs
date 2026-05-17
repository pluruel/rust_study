// Root of the library crate (crate::)
// Built as a separate crate from main.rs, producing librestaurant.rlib

fn deliver_order2() {
    println!("ORDERER")
}

// `mod X;` registers a node in the module tree and links X.rs or X/mod.rs
// Declared once, accessible anywhere in the same crate via crate::X::...
mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path: starts from the crate root
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path: based on the current module
    front_of_house::hosting::add_to_waitlist();
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    // meal.seasonal_fruit # This can not be accessed
    println!("I'd like {} toast please", meal.toast);
}
fn deliver_order() {
    println!("order!!!")
}

pub mod back_of_house {

    pub enum Appetizer {
        // enum cannot expose element only all
        Soup,
        Salad,
    }
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
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
