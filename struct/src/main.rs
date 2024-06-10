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
// enum Direction {
//     North,
//     East,
//     South,
//     West,
// }

// fn main() {
//     let my_direction = Direction::North;
//     let new_direction = my_direction;
//     move_around(new_direction);
// }

// fn move_around(direction: Direction) {
//     // implements logic to move a character around
// }

// --------------------------pattern matching-----------------------------------------
// enum Shape {
//     Circle(f64),
//     Square(f64),
//     Rectangle(f64, f64),
// }

// // Function to calculate area based on the shape
// fn calculate_area(shape: Shape) -> f64 {
//     // calculates the area of the shape
//     match shape {
//         Shape::Circle(redius) => 3.14 * redius * redius,
//         Shape::Square(side) => side * side,
//         Shape::Rectangle(width, hight) => width * hight,
//     }
// }

// fn main() {
//     // Create instances of different shapes
//     let circle = Shape::Circle(5.0);
//     let square = Shape::Square(4.0);
//     let rectangle = Shape::Rectangle(3.0, 6.0);

//     println!("Area of the circle is: {}", calculate_area(circle));
//     println!("Area of the squre is: {}", calculate_area(square));
//     println!("Area of the rectangle is: {}", calculate_area(rectangle));
// }

// ---------------------------Erooorr handling-----------------------------------------
// use std::fs;

// fn main() {
//     let res = fs::read_to_string("example.txt");
//     match res {
//         Ok(content) => {
//             println!("File content: {}", content);
//         }
//         Err(e) => {
//             println!("Error reading file: {}", e);
//         }
//     }
//     println!("Program continues...")
// }

// ---------------------Option-----------------------------------------

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main() {
    let my_string = String::from("raman");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}
