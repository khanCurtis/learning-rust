/*//UNSAFE
fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}

//Possible Fixes:
fn return_a_string() -> String {
    let s = String::from("Hello world");
    s
}

fn return_a_string() -> &'static str {
    "Hello world"
}

use std::rc::Rc;
fn return_a_string() -> Rc<String> {
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}
*/
fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str("Esq.");
    full
}

fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0]; //L1
    stringify_name_with_title(&name); //L2
    println!("{}", first); //L3
}
