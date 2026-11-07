use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    
    let t: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let c: i32 = it.next().unwrap().parse().unwrap();

        if c == 999 {
            break;
        }
        if c < -50 {
            continue;
        }

        let f = (c as f64 * 9.0 / 5.0) + 32.0;

        match f {
            x if x < 32.0 => println!("FREEZING"),
            x if x >= 32.0 && x <= 68.0 => println!("COOL"),
            x if x > 68.0 => println!("WARM"),
            _ => {}
        }
    }
}