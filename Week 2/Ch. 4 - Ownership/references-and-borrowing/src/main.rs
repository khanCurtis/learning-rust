fn main() {
    /*
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2); //L2
    let s = format!("{} {}", m1, m2); //L3 //Error: m1 & m2 are moved
    */

    /*
    let m1 = String::from("Hello");
    let m2 = String::from("world"); //L1
    let (m1_again, m2_again) = greet(m1, m2);
    let s = format!("{} {}", m1_again, m2_again); //L2
    */

    let m1 = String::from("Hello");
    let m2 = String::from("world"); //L1
    greet(&m1, &m2); //L3
    let s = format!("{} {}", m1, m2);

    //dereferencing
    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;        //*x reads heap value
    *x += 1;                //*x modifies heap value
                            //x points value to 2

    let r1: &Box<i32> = &x; //r1 points to x on the stack
    let b: i32 = **r1;      //two dereferences get us to the heap value

    let r2: &i32 = &*x;     //r2 points to the heap value directly so only 1 dereference is needed
    let c: i32 = *r2; //L1


    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x);  // explicit  dereference
    let x_abs2 = x.abs();       // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference twice
    let r_abs2 = r.abs();       // implcit dereference twice
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s);   //explicit reference
    let s_len2 = s.len();       //implicit reference
    assert_eq!(s_len1, s_len2);

    let mut v: Vec<i32> = vec![1, 2, 3]; //L1
    v.push(4); //L2     //pushes 4 to the Vector which almost acts like an array
    let num: &i32 = &v[2]; //L1     //makes the var 'num' be the 3rd item in 'v'
    println!("Third element is {}", *num); //L3


    let x = 0; //x^ +R _ -O
    let mut x_ref = &x; //x_ref^ +R +W +O   *x_ref^ +R _ _

    let mut v: Vec<i32> = vec![1, 2, 3]; //v^ +R +W +O
    let num: &i32 = &v[2]; //v> R -W -O     num^ +R _ +O    *num^ +R _ _
    println!("Third element is {}", *num);
    println!("Again, the third element is {}", *num); //v^ R +W +O      num- -R _ -O    *num- -R _ _
    v.push(4); //v- -R -W -O
    }

fn greet(g1: &String, g2: &String) /*-> (String, String)*/ {
    println!("{} {}!", g1, g2); //L2
    //(g1, g2)
}