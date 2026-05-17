use crate::garden::vegetables::Asparagus;
use restaurant::back_of_house;
use restaurant::eat_at_restaurant; // Do not use import end of leaf, use with branch is recommendable.
pub mod garden;
use std::collections::*;
use std::io::{self, Write};

use std::fmt::Result;
use std::io::Result as IoResult; // Can use as

pub use crate::front_of_house::hosting;
// use std::{cmp::Ordering, io}; // if sub-module has same parent can use like this;
fn main() {
    let asp = garden::vegetables::Asparagus { name: "가나다" };
    println!("{:?}", asp);
    eat_at_restaurant();
    back_of_house::fix_incorrect_order();
    restaurant::eat_at_restaurant();
    back_of_house::fix_incorrect_order();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 먹고 싶은 빵 바꾸기
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}
