//Parameters
fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurments(value: i32, unit_label: char) {
    println!("The measurment is: {value}{unit_label}");
}

//Statements & Expressions
fn statements() {
    //let y = 6;
    let y = {
        let x = 5;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}

fn plus_one(a: i32) -> i32 {
    a + 1
}

fn f(x: i32) -> i32 { x + 1 }

fn main() {
    let z = five();
    let a = plus_one(5);

    println!("Hello, world!");

    another_function(5);
    print_labeled_measurments(5, 'm');
    statements();

    println!("The value of z is: {z}");
    println!("The value of a is: {a}");
    println!("{}", f({
        let y = 1;
        y + 1
    }));
}
