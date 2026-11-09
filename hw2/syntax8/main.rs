use std::io;

fn main() {
    let mut first = String::new();
    io::stdin().read_line(&mut first).expect("failed to read line");
    let mut parts = first.split_whitespace();
    let _n: i64 = parts.next().unwrap().parse().unwrap();
    let k: i64 = parts.next().unwrap().parse().unwrap();

    let mut second = String::new();
    io::stdin().read_line(&mut second).expect("failed to read line");

    let mut current = 0;
    let mut best = 0;
    for token in second.split_whitespace() {
        let a: i64 = token.parse().unwrap();
        if a <= k {
            current += 1;  
        } else {
            if current > best {
                best = current;
            }
            current = 0;
        }
    }
    println!("{}", best);
}