use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: u64 = it.next().unwrap().parse().unwrap();
    let b: u64 = it.next().unwrap().parse().unwrap();
    let c: u64 = it.next().unwrap().parse().unwrap();

    let (a, b, c) = {
        let mut v = [a, b, c];
        v.sort();
        (v[0], v[1], v[2])
    };

    if a + b <= c {
        println!("Not a Triangle");
    } else if a == b && b == c {
        println!("Equilateral");
    } else if (a as u128) * (a as u128) + (b as u128) * (b as u128) == (c as u128) * (c as u128) {
        println!("Right-angled");
    } else if a == b || b == c {
        println!("Isosceles");
    } else {
        println!("Triangle");
    }
}