use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let t: usize = input.trim().parse().unwrap();

    let max_limit = 1_000_000;
    let mut count = vec![0; max_limit + 1];
    let mut sum = vec![0; max_limit + 1];

    for i in 1..=max_limit {
        count[i] = count[i - 1];
        sum[i] = sum[i - 1];
        
        if i % 3 == 0 && i % 5 != 0 {
            count[i] += 1;
            sum[i] += i as u64;
        }
    }

    for _ in 0..t {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        
        let mut it = input.split_whitespace();
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();

        let ans_count = count[b] - count[a - 1];
        let ans_sum = sum[b] - sum[a - 1];

        println!("{} {}", ans_count, ans_sum);
    }
}