fn deliver_order2() {
    println!("ORDERER")
}
mod front_of_house; // if use like this, no need to bind it

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();

    // 상대 경로
    front_of_house::hosting::add_to_waitlist();
}
