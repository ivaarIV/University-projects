use std::io;

fn main() {
    let mut t_line = String::new();
    io::stdin().read_line(&mut t_line).expect("failed to read line");
    let t: i64 = t_line.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read line");
        let mut n: i64 = line.trim().parse().expect("invalid input");

        let mut product = 1;
        let mut has_even = false;
        let mut odd_sum = 0;
        while n > 0 {
            let d = n % 10;
            if d % 2 == 0 {
                product *= d;
                has_even = true;
            } else {
                odd_sum += d;
            }
            n /= 10;
        }
        if !has_even {
            product = 0;
        }
        println!("{}", product - odd_sum);
    }
}