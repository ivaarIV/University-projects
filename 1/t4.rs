use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let n_str = input.trim();
        
        let num_digits = n_str.len() as u64;
        
        let mut sum_digits: u64 = 0;
        for c in n_str.chars() {
            sum_digits += c.to_digit(10).unwrap() as u64;
        }

        if sum_digits == num_digits * num_digits {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}