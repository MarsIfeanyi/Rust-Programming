// Bring the libraries into scope
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;
fn main() {


    println!("Guess the number!");

    // thread_rng() is an associated function that will give us random number generator
let secret_number = rand::thread_rng().gen_range(1..=100);

println!("The secret number is: {}", secret_number);

loop{
     println!("Please input your guess.");

    // creating a variable to store the user input. the "new()" returns a string. new() is a method on the string type... Hint: In Rust variables are immutable by default, thus to make them mutable we use the "mut" prefix

    let mut guess = String::new();

// gives handle to the standard  input for the current process.
    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

// converting the guess to an unsigned 32 bit integer (ie shadowing) ... parse() returns an enum which can either be "Ok" or "Error"
 let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


println!("You guessed:{}", guess);

// using the match expression to match all the possible return values
match guess.cmp(&secret_number){
    Ordering::Less=> println!("{}","Too small!".red()),
    Ordering::Greater=> println!("{}","Too big!".red()),
    Ordering::Equal=>{
        println!("{}", "You win!".green());
    break;
    },
}
 {
    
}

}


   


}
