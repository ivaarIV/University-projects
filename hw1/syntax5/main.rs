use std::io;

fn main() {
    let mut t_line = String::new();
    io::stdin().read_line(&mut t_line).expect("failed to read line");
    let t: i64 = t_line.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read line");

        let mut parts = line.split_whitespace();
        let a: i64 = parts.next().unwrap().parse().unwrap();
        let b: i64 = parts.next().unwrap().parse().unwrap();

        let mut count = 0;
        let mut sum = 0;
        for i in a..=b {
            if i % 3 == 0 && i % 5 != 0 {
                count += 1;
                sum += i;
            }
        }
        println!("{} {}", count, sum);
    }
}