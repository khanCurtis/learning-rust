const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x += 1;

    {
        let x = x * 2; // Using x *= 2 sets x globally to 12 rather just in the scope.
        println!("The value of x in this scope is: {x}");
    }

    println!("The value of x is: {x}");

    println!("Three Hours in Seconds: {THREE_HOURS_IN_SECONDS} seconds");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");
}
