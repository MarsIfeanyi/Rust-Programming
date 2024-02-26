struct User{
    username: String, 
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle{

    // Method
    fn area4(&self) -> u32
self.width * self.height    
}


fn main() {

    // creating new instance of the User struct

    let mut user1: User = User{
        email: String::from ("mars@gmail.com"),
        username: String::from ("MarsIfeanyi"),
        active: true,
        sign_in_count: 1
    };

    // We get specific values from our struct by using the dot notation
    let name: String = user1.username;

    // Updating the value in struct
user1.username = String::from("MarsEnergy");


let user2: User  = build_user(String::from("mars2@gmail.com"), String::from("HugoMars2"));


// Hint: You can create new instances of a Struct, using existing instances... Here the active and sign_in_count comes from user2
let user3:User = User{
    email:String::from("charles@gmail.com"),
    username:String::from("charles600"),
    ..user2
};


// Structs without named field... Tuple structs
struct Color(i32, i32,i32);

struct Point(i32, i32,i32);


// Calculating the area of a rectangle
let width1: u32 = 30;
let height1: u32 = 50;

println!("The area of the rectangle is {} square pixels", area(width1, height1) );


// Calculating for area2 that uses Tuple
let rect: (u32, u32) = (30,50);
println!("The area of the rectangle is {} square pixels", area2(rect) );


// Correctly calling area2 with a tuple
// println!("The area of the rectangle is {} square pixels", area2(rect) );


// creating new instance of the Rectangle struct 
let rect2: Rectangle = Rectangle{
    width: 30,
    height: 50
};
println!("The area of the rectangle is {} square pixels", area3(&rect2) );


println!("The area of the rectangle is {} square pixels", area2((rect2.width, rect2.height)) );



println!("rect: {:?}", rect); 


println!("The area of the rectangle is {} square pixels", rect.area()); // Calling the method 


}


fn build_user(email:String, username:String) -> User {
    User{
        email:email,
        username:username,
        active:true,
        sign_in_count:1
    }

}

fn build_user2(email:String, username:String) -> User {
    User{
        email,
        username,
        active:true,
        sign_in_count:1
    }

}


fn area(width:u32, height: u32) -> u32 {
    width * height

}

// Modifying it to use Tuple
fn area2(dimensions: (u32, u32)) ->u32 {
    dimensions.0 * dimensions.1
}

// Modifying the function to use the fields defined in the Rectangle Struct
fn area3(rectangle: &Rectangle) -> u32{
    rectangle.width * rectangle.height
}