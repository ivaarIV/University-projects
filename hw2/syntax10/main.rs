use std::io;

fn fib(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    let mut t_line = String::new();
    io::stdin().read_line(&mut t_line).expect("failed to read line");
    let t: i64 = t_line.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read line");
        let n: u64 = line.trim().parse().expect("invalid input");
        println!("{}", fib(n));
    }
}