// Bringing the library into scope
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;


fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

     println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

// Hint: by default variables in Rust are immutable, thus we use the "mut" prefix to make them mutable. 
        let mut guess = String::new();

// The input/output functionality, allows us to handle user inputs... The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), The "&" indicates that this argument is a reference ie a mutable reference(&mut)
        io::stdin()
            .read_line(&mut guess) 
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //  The underscore, _, is a catchall value
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{} ", "Too small!".red()),
            Ordering::Greater => println!( "{}" ,"Too big!".red()),
            Ordering::Equal => {
                println! ("{}",   "You win!".green());
                break;
            }
        }
    }
}