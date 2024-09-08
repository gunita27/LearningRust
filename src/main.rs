use rand::Rng; //rand is a crate that provides the random number generator
use std::cmp::Ordering;
use std::io;
fn main() {
    let correct = rand::thread_rng().gen_range(1..=10);
    // println!("correct:{correct}");
    println!("Hello there ");
    println!("Guesss a number from 1-10");
    loop {
        let mut guess = String::new(); //keep this inside the loop otherwise it will throw error as it will store the value in the memory once and then it will be overwritten
                                       //std is standard library
        io::stdin()
            .read_line(&mut guess) //read_line is used to read the input from the user
            .expect("error reading input");
        // when you use expect it just ignores the error and continues the execution so for a little more good code like before when we added negative number it just break the loop but now it gives us the error message that hey you did this wrong you entered invalid number :

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("error with parse,try again {e}");
                continue;
            }
        };
        //parse is used to convert string to integer
        //expect is used to handle error

        //there's a different way to do this as well lemme show you
        /* instead of using println 3 times we can do it in one statement  */
        // this is the first way
        // if correct >guess{
        //     println! (

        //       "you picked smaller number"
        //     );}
        //     else if correct < guess {
        //       println!(
        //         "you picked greater number"
        //       );

        // }else{
        //     println!("you chose the correct number");
        // }
        // this is now second way of doing this
        // let mut message=if correct >guess{
        //   String::from(
        //     "you picked smaller number"
        //   )}
        //   else if correct < guess {
        //    String::from (
        //       "you picked greater number"
        //     )

        // }else{
        //   String::from("you chose the correct number")
        // };
        // Caleb carry says its more rust like to do this uk what we do the simple if else statement is in c/c++ as well which i have learned but for me this is what makes rust more interesting and fun to use i love it!! Yeah i am seeing yt tutorial rn but i am reading rust book as well and he is taking reference from the book only i yap alot.

        // println!("You guessed {} number", name.trim());//trim cant be used with integer its for string
        // now we are going to use match statement
        // so this is the third way of doing this and yeah so cmp compares the two values and returns the value of the comparison like here it compares guess to correct and if its greater less or equal it prints the values below how amazing it is right? from using variable just for storing values we are here using conditions in variable
        // let message=match guess.cmp(&correct){
        //   Ordering::Greater=>"you picked greater number",
        //   Ordering::Less=>"you picked smaller number",
        //   Ordering::Equal=>{ println!("you chose the correct number");
        //                     break;

        //     }
        // };
        // println!("{message}");
        match guess.cmp(&correct) {
            Ordering::Greater => println!("you picked greater number"),
            Ordering::Less => println!("you picked smaller number"),
            Ordering::Equal => {
                println!("you chose the correct number");
                break;
            } // we can also just remove the variable
        };
    }
}
