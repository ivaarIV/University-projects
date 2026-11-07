use std::io;

fn main() {
    let mut t_line = String::new();
    io::stdin().read_line(&mut t_line).expect("failed to read line");
    let t: i64 = t_line.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read line");
        let mut n: i64 = line.trim().parse().expect("invalid input");

        let mut sum = 0;
        let mut count = 0;
        while n > 0 {
            sum += n % 10;
            count += 1;
            n /= 10;
        }

        if sum == count * count {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}