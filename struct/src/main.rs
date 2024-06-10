// struct User {
//     name: String,
//     user_name: String,
//     age: u32,
//     active: bool,
//     number: u32,
//     email: String,
// }

// fn main() {
//     let user: User = User {
//         name: String::from("Akash"),
//         user_name: String::from("ak123"),
//         age: 22,
//         active: true,
//         number: 145345,
//         email: String::from("akash**@gmail.com"),
//     };

//     println!("Name: {}", user.name);
//     println!("User Name: {}", user.user_name);
//     println!("Age: {}", user.age);
//     println!("Active: {}", user.active);
//     println!("Number: {}", user.number);
//     println!("Email: {}", user.email);
// }

// -----------traits-----------------------------------------

// #[derive(Clone, Copy)]
// struct User {
//     age: u32,
//     number: u32,
// }

// fn main() {
//     let user1 = User {
//         age: 22,
//         number: 145345,
//     };

//     print_user(user1);
//     println!("Age: {}", user1.number);
// }

// fn print_user(user: User) {
//     println!("Age: {}", user.age);
// }

// ---------------------------Implementing structs--------------------------
// struct Rect {
//     width: u32,
//     height: u32,
// }

// impl Rect {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

// fn main() {
//     let rect = Rect {
//         width: 30,
//         height: 50,
//     };
//     print!("The area of the rectangle is {}", rect.area());
// }

// --------------enums--------------------------------------------------------------/
// Define an enum called Shape
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let my_direction = Direction::North;
    let new_direction = my_direction;
    move_around(new_direction);
}

fn move_around(direction: Direction) {
    // implements logic to move a character around
}
