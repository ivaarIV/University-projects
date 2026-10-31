use std::io;

fn main() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("failed to read line");

    let mut age_line = String::new();
    io::stdin().read_line(&mut age_line).expect("failed to read line");
    let age: i64 = age_line.trim().parse().expect("invalid input");

    println!("Hello! My name is {} and I'm {} years old.", name.trim(), age);
}