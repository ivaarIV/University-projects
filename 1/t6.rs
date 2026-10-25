use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n_str = input.trim();

    let mut sum: i64 = 0;
    let mut sign: i64 = 1;

    for c in n_str.chars() {
        let digit = c.to_digit(10).unwrap() as i64;
        sum += digit * sign;
        sign *= -1;
    }

    println!("{}", sum);
}