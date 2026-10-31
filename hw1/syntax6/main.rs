use std::io;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line");
    let mut n: i64 = line.trim().parse().expect("invalid input");

    let mut div = 1;
    let mut m = n;
    while m >= 10 {
        m /= 10;
        div *= 10;
    }

    let mut sum = 0;
    let mut sign = 1;
    while div > 0 {
        let digit = n / div;
        sum += sign * digit;
        sign = -sign;
        n %= div;
        div /= 10;
    }
    println!("{}", sum);
}