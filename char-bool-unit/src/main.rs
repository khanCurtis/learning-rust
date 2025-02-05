use std::mem::size_of_val;

fn main() {
    //1. 
    let c1: char = 'a'; //size of letter 'a' is 4 bytes
    assert_eq!(size_of_val(&c1),4);

    println!("{}", size_of_val(&c1));
    
    //2. 
    let c2: char = 'T';
    assert_eq!(size_of_val(&c2),4);

    println!("Success!");

    //3. 
    let c3: char = 'T';
    print_char(c3);

    println!("Success!");

    //4. 
    let f: bool = false;
    let t: bool = true && false; //false
    assert_eq!(t, f);

    println!("Success!");

    //5. 
    let _v: () = ();

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");

    //6. 
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");

}

fn print_char(c: char) {
    println!("{}", c);
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

