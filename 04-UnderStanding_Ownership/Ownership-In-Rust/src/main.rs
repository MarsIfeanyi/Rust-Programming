fn main(){

let s:String = String::from ("hello Mars");
// takes_ownership(s);
println!("{}", s);

let x: i32 =5;
makes_copy(x);
println!("{}", x);

let s1:String = gives_ownership();
println!("s1 = {}", s1);


let s2:String = String::from("hello");
let s3:String = takes_and_gives_back(s2);
println!("s1 = {}, s3={}", s1,s3);


let len:usize = calculate_length(&s1);
println!("The length of '{}' is {}.", s1, len);

let explain = explanations();


}


fn explanations(){

  // ----- Ownership Rules -----

    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // Hint: String literals are stored directly in binary and they are fixed in size

    // { // s is not valid here, it's not yet declared
    // let s:String = String::from ("hello") // s is valid from this point forward
    // // do stuff with s 

    // } // this scope is now over, and s is no longer valid.



    let x: i32 = 5;
let y:i32 = x; // Copy

let s1:String = String::from ("hello");

 // let s2:String = s1; // Move (not shallow copy)

 // Hint: Rust default to moving a value thus if you want to clone you will use the clone()
// to clone the string 
let s2 = s1.clone();

println!("{} World!", s1 );


}


fn takes_ownership(some_string: String){
    println!("{}", some_string);

    // Hint: Strings are not copied directly
}

fn makes_copy(some_integer: i32){
    println!("{}", some_integer);

    // Hint: Integers are copied directly
}

fn gives_ownership() -> String{
    let some_string: String = String::from ("hello gives ownership");

    some_string
}

fn takes_and_gives_back(a_string:String) -> String{
    a_string
}

fn calculate_length(s:&String) ->usize {
    // Hint: references don't take ownership of the underlying values ... the `&` is used to denote references... references are immutable by default

    let length:usize=s.len(); // len() returns the length of a string
    length



    // The Rules of references
    // 1. At any given time, you can have either one mutable reference or any number of immutable references
   // 2. References must always be valid 

}