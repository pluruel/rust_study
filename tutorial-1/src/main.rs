use rand;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");
    let secret_num = rand::random_range(1..=100);
    println!("secret num is : {secret_num}");

    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed_read line");
        println!("your guess is {guess}");
        // let guess: u32 = guess.trim().parse().expect("Please type number");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("small!!"),
            Ordering::Greater => println!("Big!!"),
            Ordering::Equal => {
                println!("SAME!");
                break;
            }
        }
    }
}
