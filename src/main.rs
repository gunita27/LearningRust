use std::io;
fn main() {
    println!("Hello there ");
    println!("Guesss a number");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("error reading input");
    println!("You guessed {name} number");
    println!("You guessed {} number",name.trim());
}