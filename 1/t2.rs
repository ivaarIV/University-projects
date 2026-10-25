use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let a: u64 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: u64 = input.trim().parse().unwrap();

    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let c: u64 = input.trim().parse().unwrap();

    let mut sides = [a, b, c];
    sides.sort();

    let x = sides[0];
    let y = sides[1];
    let z = sides[2];

    if x + y <= z {
        println!("Not a Triangle");
    } else if x == y && y == z {
        println!("Equilateral");
    } else if x * x + y * y == z * z {
        println!("Right-angled");
    } else if x == y || y == z {
        println!("Isosceles");
    } else {
        println!("Triangle");
    }
}