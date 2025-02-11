fn main() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", numbers);
    
    let fruits: [&str; 3] = ["pineapple", "pomegranate", "coconut"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st El: {}", fruits[0]);
    println!("Fruits Array 2nd El: {}", fruits[1]);
    println!("Fruits Array 3rd El: {}", fruits[2]);

    //Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuple: {:?}", human);

    let mixed_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);
    println!("My Mix Tuple: {:?}", mixed_tuple);

    //Slices: [1,2,3,4,5]
    let number_slices: &[i32] = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Snake", "Frog", "Dog"];
    println!("Animal Slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"Teacher".to_string(), &"Hunger Games".to_string(), &"Harry Potter".to_string()];
    println!("Book Slices: {:?}", book_slices);

}

