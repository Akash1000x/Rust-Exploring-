fn main() {
    println!("Hello Rust!")
}

// -------------------------------------------------------------------------

// fn main() {
// let greeting = String::from("Hello World!");
// // print!("{}", greeting);

// let char1 = greeting.chars().nth(1);

// match char1 {
//     Some(c) => print!("{}", c),
//     None => print!("No character at index 100"),
// }

// print!("{}", greeting.chars().nth(1).unwrap())

// let is_male = true;
// let is_above_18 = true;

// if is_male {
//     println!("You are a male");
// } else {
//     println!("You are not a male");
// }

// if is_male && is_above_18 {
//     print!("You are a legal male");
// }

// let x = 4;
// let y = 5;
// println!("{}", x + y);

// let mut x = 100;
// for _i in 0..1000 {
//     x = x + 100;
// }
// print!("{}", x);
// }

//mutabality -------------------------------------------------------------------------

// fn main() {
//     // for mutable data use mut
//     let mut s1 = String::from("Hello");
//     println!(
//         "length {}, Capacity: {}, MemoryAddress: {:p} ",
//         s1.len(),
//         s1.capacity(),
//         s1.as_ptr()
//     );
//     s1.push_str(" World!");
//     println!("{}", s1);
//     println!(
//         "length {}, Capacity: {}, MemoryAddress: {:p} ",
//         s1.len(),
//         s1.capacity(),
//         s1.as_ptr()
//     );
// }

//-------------Ownership--------------------------------------------------

// fn main() {
//     let s1 = String::from("Akash");
//     let s2 = s1;
//     println!("{}", s1); // s1 is not have the data anymore
//     print!("{}", s2);
// }

// fn main() {
//     let my_string = String::from("Akash");
//     take_string(my_string);
//     println!("{}", my_string); // now this is not valid because it goes to the take_string function
// }

// fn take_string(some_string: String) {
//     println!("{}", some_string);
// }

// ---------one solution for the above problem is--------------
// fn main() {
//     let mut my_string = String::from("Akash");
//     my_string = take_string(my_string);
//     println!("{}", my_string);
// }

// fn take_string(some_string: String) -> String {
//     println!("{}", some_string);
//     return some_string;
// }

// ----------other solution-----
// fn main() {
//     let my_string = String::from("Akash");
//     take_string(my_string.clone()); //just clone the string
//     println!("{}", my_string);
// }

// fn take_string(some_string: String) {
//     println!("{}", some_string);
// }

// ------------pass by references----------------------

// fn main() {
//     let s1 = String::from("Hello");
//     update_string(&s1);
//     println!("{}", s2);
// }
// fn update_string(some_string: &String) {
//     println!("{}", some_string)
// }

// ------------pass by references and mutable---------------------
// fn main() {
//     let mut s1 = String::from("Hello");
//     update_string(&mut s1);
//     println!("{}", s2);
// }

// fn update_string(some_string: &mut String) {
//     some_string.push_str(" Rust");
//     println!("{}", some_string)
// }

// fn main() {
//     let mut s1 = String::from("Hello");
//     update_string(&mut s1);
//     let s2 = &mut s1;
//     println!("{}", s1);// don't have many immutable refference
//     println!("{}", s2);
// }

// fn update_string(some_string: &mut String) {
//     some_string.push_str(" Rust");
//     println!("{}", some_string)
// }
