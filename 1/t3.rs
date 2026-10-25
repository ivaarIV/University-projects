use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut n: u64 = input.trim().parse().unwrap();

        let mut length = 1;

        while n != 1 {
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = 3 * n + 1;
            }
            length += 1;
        }

        println!("{}", length);
    }
}