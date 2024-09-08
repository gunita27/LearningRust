use rand::Rng;//rand is a crate that provides the random number generator
use std::io;
fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    println!("correct:{correct}");
    println!("Hello there ");
    println!("Guesss a number from 1-10");
    let mut guess = String::new();
//std is standard library
    io::stdin()
        .read_line(&mut guess)//read_line is used to read the input from the user
        .expect("error reading input");
  let guess: u32 = guess.trim().parse().expect("error with parse");//parse is used to convert string to integer
  //expect is used to handle error
    if correct >guess{
        println! (
          
          "you picked smaller number"
        );}
        else if correct < guess {
          println!(
            "you picked greater number"
          );
        
    }else{
        println!("you chose the correct number");
    }
  
    // println!("You guessed {} number", name.trim());//trim cant be used with integer its for string
}
