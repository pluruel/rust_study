fn main() {
    let mut d = String::from("test");

    d.push_str("fffff");

    println!("ddddd: {d}");

    let d2 = d; // shallow copy, and d lost it's owenability
    let d3 = d2.clone(); // Deep copy
    // println!("D222222: {d}"); it can't be run
    println!("D222222: {d2}");
    println!("D333333: {d3}");

    // d2[0] = '김';

    let s3 = 0;
    let s4 = s3; // deep copy

    let s = String::from("hello"); // s가 스코프 안으로 들어옵니다

    takes_ownership(s); // s is moved!!!
    let s = String::from("DDDd");
    println!("{s}");

    let len = calculate_length(&s);
    println!("{s}, {len}"); // still can use, & is borrow.

    let mut s2 = String::from("hello");

    let r1 = &s2; // 문제없음
    let r2 = &s2; // 문제없음
    println!("{} and {}", r1, r2);

    let reference_to_nothing = dangle();
    println!("{}", reference_to_nothing);

    let mut s3 = String::from("Hello world");
    let res = first_word(&s3);
    let hello = &s3[0..5];
    let world = &s3[6..11];

    println!("{} {}", hello, world);
    hello.clear();
    println!("{}", s3);
    s3.clear();
    // println!(s2);
}

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn dangle() -> String {
    let s = String::from("hello2222");

    s
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
// must define Return type.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
