use std::io;

fn main() {
    let mut t_line = String::new();
    io::stdin().read_line(&mut t_line).expect("failed to read line");
    let t: i64 = t_line.trim().parse().expect("invalid input");

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line");

    for token in line.split_whitespace() {
        let c: i32 = token.parse().unwrap();

        if c < -50 {
            continue;
        }
        if c == 999 {
            break;
        }

        let f = c as f64 * 9.0 / 5.0 + 32.0;

        match f {
            x if x < 32.0 => println!("FREEZING"),
            x if x >= 32.0 && x <= 68.0 => println!("COOL"),
            _ => println!("WARM"),
        }
    }
}