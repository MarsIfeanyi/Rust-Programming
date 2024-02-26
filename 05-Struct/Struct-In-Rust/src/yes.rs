struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut user1: User = User {
        email: String::from("mars@gmail.com"),
        username: String::from("MarsIfeanyi"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("MarsEnergy");

    let user2: User = build_user(String::from("mars2@gmail.com"), String::from("HugoMars2"));

    let user3: User = User {
        email: String::from("charles@gmail.com"),
        username: String::from("charles600"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let width1: u32 = 30;
    let height1: u32 = 50;

    println!("The area of the rectangle is {} square pixels", area(width1, height1));

    let rect: (u32, u32) = (30, 50);
    println!("The area of the rectangle is {} square pixels", area2(rect));

    let rect2: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels", area3(&rect2));

    println!("The area of the rectangle is {} square pixels", area2((rect2.width, rect2.height)));
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
