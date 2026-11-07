use std::io;
fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    io::stdin().read_line(&mut a).expect("failed to read line");
    io::stdin().read_line(&mut b).expect("failed to read line");
    io::stdin().read_line(&mut c).expect("failed to read line");

    let a: i64 = a.trim().parse().expect("invalid input");
    let b: i64 = b.trim().parse().expect("invalid input");
    let c: i64 = c.trim().parse().expect("invalid input");

    if a + b <= c || a + c <= b || b + c <= a {
        println!("Not a Triangle");
    } else if a == b && b == c {
        println!("Equilateral");
    } else if a * a + b * b == c * c || a * a + c * c == b * b || b * b + c * c == a * a {
        println!("Right-angled");
    } else if a == b || b == c || a == c {
        println!("Isosceles");
    } else {
        println!("Triangle");
    }
}