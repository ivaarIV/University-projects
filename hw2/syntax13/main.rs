use std::io;

fn process_token(c: char) -> Option<i64> {
    match c {
        '+' => Some(1),
        '-' => Some(-1),
        '*' => Some(2),
        '#' => None,
        _ => Some(0),
    }
}

fn main() {
    let mut t_line = String::new();
    io::stdin().read_line(&mut t_line).expect("failed to read line");
    let t: i64 = t_line.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read line");

        let mut total = 0;
        for ch in line.trim().chars() {
            match process_token(ch) {
                Some(0) => continue,
                Some(v) => total += v,
                None => break,
            }
        }
        println!("{}", total);
    }
}