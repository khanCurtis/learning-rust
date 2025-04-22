fn main() {
    let condition = false;
    let num = if condition {5} else {6};
    let mut bool;
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    let mut count = 0;

    println!("The result is {result}");

    //if Expressions
    if num < 5 {
        bool = true
    } else {
        bool = false
    }
    if num != 0 {
        println!("The number is not zero");
    }
    if num % 4 == 0 {
        println!("The number is divisible by 4");
    } else if num % 3 == 0 {
        println!("The number is divisible by 3");
    } else if num % 2 == 0 {
        println!("The number is divisible by 2");
    } else {
        println!("The number is not divisible by 4, 3, or 2");
    }

    println!("bool is {bool}");

    //loop
    /*
    loop {
        print!("again!");
    }
*/
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    //while loops
    let mut number = 3;
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    //for loops
    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
