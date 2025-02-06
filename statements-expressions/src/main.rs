fn main() {
    let suc: &str = "Success!";

    let x: u32 = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x    
    };
    let z: u32 = {
        2 * x
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    //1. 
    let v: i32 = {
        let mut w: i32 = 1;
        w += 2; //3
        w
    };
    
    assert_eq!(v, 3);

    println!("{}", suc);

    //2. 
    let a = {
        let b = 3;
        b
    };

    assert!(a == 3);

    println!("{}", suc);

    //3. 
    let s: i32 = sum(1, 2);
    assert_eq!(s, 3);

    println!("{}", suc);

}

fn sum(c: i32, d: i32) -> i32 {
    c + d
}