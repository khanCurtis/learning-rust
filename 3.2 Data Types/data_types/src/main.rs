fn main() {
    let _x = 2.0;    // f64
    let _y: f32 = 3.0;    // f32

    let _sum = 5 + 10;               // addition
    let _difference = 95.5 - 4.3;    // subtraction
    let _product = 4 * 30;           // multiplication
    let _quotient = 56.7 / 32.2;     // division
    let _truncated = -5 / 3;         // division
    let _remainder = 43 % 5;         // remainder

    let _t = true;
    let _f: bool = false;

    let _c = 'z';
    let _z: char = 'Z';  // with explicit type annotation

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;

    println!("The value of y is: {y}");
}
