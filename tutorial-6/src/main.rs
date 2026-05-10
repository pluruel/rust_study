use std::u8;

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    let d = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{}", d);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 8;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => move_player(), // _ can not use as variable
    }
    println!("{}", dice_roll);

    let config_max = Some(255u16);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // the same as
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }
}
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
impl Message {
    fn call(&self) {
        println!("{:?}", self)
    }
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Totally enum is make for use as one element, but has multiple shape
// Especially "Option" is good implements.
// enum Option<T> {
//     None,
//     Some(T),
// }

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player() {
    println!("DDDFFF???")
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_state) => 25,
    }
}

enum UsState {
    Alabama,
    Alaska,
    // --생략--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
