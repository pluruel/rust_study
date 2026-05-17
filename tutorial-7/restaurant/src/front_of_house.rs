// The `mod front_of_house;` declaration in lib.rs locates and links this file
// The filename must match the module name (enforced by the compiler)
//
// As the module grows, it can be expanded into a folder:
//   front_of_house.rs            (keeps only declarations: `pub mod hosting;`)
//   front_of_house/hosting.rs    (body moved out)

pub mod hosting {
    pub fn add_to_waitlist() {
        println!("DDDD2??!!!");
    }
}

// pub mod front_of_house {
//     use super::deliver_order2;
//     pub mod hosting {

//         pub fn add_to_waitlist() {
//             println!("DDDD??!!!");
//             super::deliver_order2();
//         }

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }
