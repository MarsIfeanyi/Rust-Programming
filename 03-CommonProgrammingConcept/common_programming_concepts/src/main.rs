fn main() {

// VARIABLES

    // In Rust variables are immutable by default

    let mut  x: i32 = 5;
    println!("The value of x is: {}", x);

    x = 6;
    println!("The value of x is: {}", x);

    // Declaring Constant variables. Hint: You cannot mutate a constant

    const SUBSCRIBER_COUNT:u32 = 100_000;

    // SHADOWING: It allows you to create a new variable, using an existing name.
let   y: i32 = 5;
    println!("The value of y is: {}", y);

    // Hint: The first y variable is shadowed by the 2nd y variable. 
  let   y:&str ="Six" ;
    println!("The value of y is: {}", y);
    
}
