#[allow(unused_variables)]
fn main() {
    //1. initialize var
    let x: i32 = 5;
    let y: i32;

    assert_eq!(x, 5);
    println!("Success!");

    //2. use mut to mark a var as mutable
    let mut z: i32 = 1;
    z += 2;

    assert_eq!(z, 3);
    println!("Success!");

    //3. scope
    let a: i32 = 10;
    let b: i32 = 5;

    { //this is a scope
        //let b: i32 = 5;
        println!("inside scope The value of a is {} and the value of b is {}", a, b);
    }
    println!("The value of a is {} and the value of b is {}", a, b);

    //4. functions
    define_c();

    //5. shadowing
    let d: i32 = 5;
    {
        let d = 12;
        assert_eq!(d, 12);
    }

    assert_eq!(x, 5);

    let d: i32 = 42;
    println!("{}", d);

    //6. 
    let mut e: i32 = 1;
    e = 7;
    let mut e = e;
    e += 3;


    let f: i32 = 4;
    let f: &str = "I can also be bound to text!";

    println!("Success!");

    //7. unused var
    let _g = 1; //add the _ to get rid of the warning

    //8. Destructering
    let (mut h, i) = (1, 2);
    h += 2;

    assert_eq!(h, 3);
    assert_eq!(i, 2);

    println!("Success!");

    //9. 
    let (j, k);

    (j,..) = (3, 4);
    [.., k] = [1, 2];

    assert_eq!([j,k], [3, 2]);
    println!("Success!");
}

fn define_c() {
    let c: &str = "hello";

    println!("{}, world!", c);
}

