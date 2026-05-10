fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}", user1.email);
    println!("{:#?}", user1);
    // dbg need to use as reference type.
    dbg!(&user1);

    let rect = Rectangle {
        width: 1,
        height: 2,
    };

    let rect2 = Rectangle {
        width: 4,
        height: 6,
    };
    let rr = Rectangle::square(3);

    println!("rrrrr, {}", rect.area());
    println!("rrrrr, {}, {}", rect.width(), rect.width);
    println!("CANHOLD : {}", rect2.can_hold(&rect));
    dbg!(rr);
}

// If use this "outer attribute", we can see elemets in struct directly.
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
