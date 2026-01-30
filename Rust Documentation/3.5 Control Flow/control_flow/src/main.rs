fn main() {
    let num = 7;

    if num < 5 {
        println!("Num is less than 5!");
    } else {
        println!("Num is more than 5!");
    }

    if num != 0 {
        println!("Num is not zero");
    }

    match num {
        n if n % 4 == 0 => println!("Num is divisible by 4"),
        n if n % 3 == 0 => println!("Num is divisible by 3"),
        n if n % 3 == 0 => println!("Num is divisible by 3"),
        _ => println!("Num is not divisible by 4, 3, or 2!"),
    }

    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of number is: {num}");

    let mut i = 0;
    loop {
        println!("again!");
        i += 1;

        if i > 5 {
            break;
        }
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is: {result}");

    let mut count = 0;
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

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("Liftoff!!");

    let a = [10, 20, 30, 40, 50];
    let mut i = 0;

    while i < 5 {
        println!("The value is: {}", a[i]);
        i += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for num in (1..4).rev() {
        println!("{num}!");
    }
    println!("LIFTOFF!!");
}
