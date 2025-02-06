fn main() {
    let suc: &str = "Success!";

    //1. 
    let (x, y) = (1, 2);
    let s: i32 = sum(x, y);

    assert_eq!(s, 3);

    println!("{}", suc)




}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}