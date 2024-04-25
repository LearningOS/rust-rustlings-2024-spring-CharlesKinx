// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {number}");
    let binding = String::from("3");
    number = &binding; // don't rename this variable
    let binding = (number.to_owned() + "2");
    number = &binding;
    println!("Number plus two is : {number}");
}
