fn main() {
    let greeting = String::from("Hello World!");
    // print!("{}", greeting);

    let char1 = greeting.chars().nth(1);

    match char1 {
        Some(c) => print!("{}", c),
        None => print!("No character at index 100"),
    }

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
}
