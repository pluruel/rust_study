use std::io;

fn main() {
    let mut x = 5;
    x += 3;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("the size of spaces is {spaces}");

    let tup: (i32, f64, u8) = (500, 0.1, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);
    // println!(tup.0, tup.1, tup.2); This pattern is not working
    // println!("{tup.0}, {tup.1}, {tup.2}"); as well

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // add type like this style

    let a = [3; 5]; // [5,5,5,5,5]
    let first = a[0] + 1;
    println!("first item: {first}");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    another_function();
    print_labeled_measurement(3, '감');
    // let x = (let y = 6); Cannot compile it's statement so it can be stored.
    let y = {
        let x = 3;
        x + 1 // IMPORTANT: there is no semicolon. it means "return"
    };
    let d = {
        // let y = 3 statement cannot be expression
        let y = 3;
        y + 1 // That is expression
    }; // this block also expression
    println!("The value of y is: {y}");

    if d < 5 {
        println!("condition is good")
    } else {
        println!("condition is bad!")
    }
    // if d {} if condition must be bool!!!
    let number = if d < 5 { 5 } else { 6 }; // be careful return with EXPRESSION not statement.
    // let number = if d < 5 { 5 } else { "six" };  and the type must be same
    println!("The value of number is: {number}");

    // loop {
    //     println!("TEST!!!")
    // } unlimited loop

    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("counter : {counter}");
        if counter == 10 {
            break counter * 2; // this can be also statement?
        } // no semicolon > statement > return value (not expression!!!)
    };
    println!("final count: {result}");

    let mut cnt = 0;
    'counting_loop: loop {
        // must be defined loop start from '
        println!("count: {cnt}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if cnt == 2 {
                break 'counting_loop;
            }
            remaining -= 1;
        }
        cnt += 1;
    }
    println!("End count = {cnt}");
    let mut number = 3;

    // while is simillar with python
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for element in (1..4).rev() {
        println!("count down : {element}")
    }
}

fn another_function() {
    println!("Another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}
