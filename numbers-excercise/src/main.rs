/*
Signed integer can represent both positive and negative intgers
Unsigned integers are always positive

Length  Signed  Unsigned
8-bit   i8      u8
16-bit  i16     u16
32-bit  i32     u32
64-bit  i64     u64
128-bit i128    u128
arch    isize   usize //based on computer architecture i.e. 64x arch = 64-bit

integers: i32
floats: f64
*/

fn main() {
    //1.
    let x: i32 = 5;
    let mut y/*: u32*/ = 5;

    y = x; //can't combine 2 integers of 2 different types

    let z: i32 = 10;

    println!("Success!");

    //2. 
    let v: u16 = 38_u8 as u16;

    println!("Success!");

    //3. 
    let z: u32 = 5;
    assert_eq!("u32".to_string(), type_of(&z));

    println!("Success!");

    //4. 
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");

    //5. 
    let v1: u16 = 251_u16 + 8;
    let v2: i16 = i16::checked_add(251, 8).unwrap();
    
    println!("{}, {}", v1, v2);

    //6. 
    let a = 1_024 + 0xff + 0o77 + 0b1111_1111; //1024 + 255 + 63 + 255
    assert!(a == 1597);

    println!("Success!");

    //7. 
    let b: f64 = 1_000.000_1;
    let c: f32 = 0.12;
    let d: f64 = 0.01_f64;

    assert_eq!(type_of(&b), "f64".to_string());
    println!("Success!");

    //8. 
    assert!(0.1 as f32 + 0.2_f32 == 0.3_f32); //0.1 + 0.2 = 0.3000000000x

    println!("Success!");

    //9. 
    let mut sum: i32 = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for e in 'a'..='z' {
        println!("{}", e as u8);
    }

    //10. 
    use std::ops::{Range, RangeInclusive};

    assert_eq!((1..5), Range{ start: 1, end: 5});
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");

    //11. 
    //addition
    assert!(1u32 + 2 == 3);
    
    //subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    //multiplication
    assert!(3 * 50 == 150); //default: i32

    //division
    assert!(9.6 as f32 / 3.2 == 3.0);

    //modulus
    assert!(24 % 5 == 4);

    //short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    //bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

//gets the type of a given var, returns a string representation of the type
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

