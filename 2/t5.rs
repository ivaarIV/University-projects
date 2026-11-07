use std::io;

fn process_token(c: char) -> i32 {
    match c {
        '+' => 1,
        '-' => 1,
        '*' => 2,
        _ => 0,
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let s = input.trim();

        let mut sum = 0;

        for c in s.chars() {
            if c == '#' {
                break;
            }
            
            if c == '+' || c == '-' || c == '*' {
                sum += process_token(c);
            } else {
                continue;
            }
        }

        println!("{}", sum);
    }
}