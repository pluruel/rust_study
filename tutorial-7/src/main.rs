use crate::garden::vegetables::Asparagus;
use restaurant::eat_at_restaurant; // Do not use import end of leaf, use with branch is recommendable.
pub mod garden;
use std::collections::*;
use std::io::{self, Write};
// use std::{cmp::Ordering, io}; // if sub-module has same parent can use like this;
fn main() {
    let asp = garden::vegetables::Asparagus { name: "가나다" };
    println!("{:?}", asp);
    eat_at_restaurant();
}
