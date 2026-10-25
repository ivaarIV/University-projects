use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    
    let t: usize = it.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n_str = it.next().unwrap();

        let mut even_product: i64 = 1;
        let mut odd_sum: i64 = 0;

        for c in n_str.chars() {
            let digit = c.to_digit(10).unwrap() as i64;
            if digit % 2 == 0 {
                even_product *= digit;
            } else {
                odd_sum += digit;
            }
        }

        let result = even_product - odd_sum;
        println!("{}", result);
    }
}