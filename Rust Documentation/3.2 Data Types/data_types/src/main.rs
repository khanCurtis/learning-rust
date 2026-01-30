use std::io::{self, Write};

fn main() {
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    let _sum = 5 + 10; // addition
    let _difference = 95.5 - 4.3; // subtraction
    let _product = 4 * 30; // multiplication
    let _quotient = 56.7 / 32.2; // division
    let _truncated = -5 / 3; // division
    let _remainder = 43 % 5; // remainder

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z: char = 'Z'; // with explicit type annotation

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");

    let _x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = _x.0;
    let _six_point_four = _x.1;
    let _one = _x.2;

    let _a = [1, 2, 3, 4, 5];
    let _months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let _first = a[0];
    let _second = a[1];

    print!("Please enter an array index: ");
    io::stdout().flush().unwrap();
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a nubmber");
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
