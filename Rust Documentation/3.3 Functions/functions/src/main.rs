fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_measurement(value: i32, label: char) {
    println!("The measurement is: {value}{label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, World!");

    another_function(5);
    print_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

    let x = plus_one(5);
    println!("The value of x is: {x}");
}
