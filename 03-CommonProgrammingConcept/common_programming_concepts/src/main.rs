fn main() {

    // Calling the function

let sum: i32 = my_function(x:11, y:22);
println!("The sum is: {}", sum)

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




// DATA types

// Scalar data types
// 1. Integers
// 2. Floating-point numbers (numbers with Decimal points)
// 3. Booleans
// 4. Character


let a: i32 =98_222; // Decimal
let b: i32 = 0xff; // Hex
let c: i32 = 0o77; // Octal
let d: i32 = 0b1111_0000; // Binary
let e: u8 = b'A'; // Byte (u8 only)

// let f:u8 = 256; // will lead to integer overflow


// Floating-point numbers 

let f:f64 = 2.0;
let g: f32 = 3.0;

// addition
let sum: i32 = 5 + 10;
 // subtraction
 let difference: f64 = 95.5 - 4.3;
 // multiplication
 let product: i32 = 4 * 30;
 // division
 let quotient: f64 = 56.7 / 32.2;
// remainder
let remainder: i32 = 43 % 5;


// Booleans 

let t:bool = true;

let f:bool = false;


// Character

let c:char = 'z';
let z:char = 'Z';

// Compound Data types: They represent a group of values

// TUPLE

let tup: (&str, i32) = ("Let's Get Rusty!", 100_000); // tuples are written by using commas separated list inside a parenthesis

// We can get values out of tuple by destructuring and by using dot notation

let (channel, sub_count) = tup; // destructuring

let sub_count: i32 = tup.1;


// ARRAYS

let error_codes: [i32; _]= [200, 404,500]; // Arrays are zero index based

let not_found:i32 = error_codes[1];

let x: i32 = error_codes[3];

// Another way of Declaring an array
let byte:[i32; _] = [0;8]; // create an array with 8 values or set to 0




    
}


// Declaring Functions. Rust uses the snake case for function naming.

// You will invoke or call the function you declared inside the main function

fn my_function(x: i32, y:i32) -> i32 {
    println!("Another function");

    println!("The value of x is: {}", x);
     println!("The value of y is: {}", y);

     x + y;

}
