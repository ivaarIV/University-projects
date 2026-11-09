use std::io;

fn find_first_vowel(s: &str) -> &str {
    for (index, ch) in s.char_indices() {
        match ch.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => return &s[index..],
            _ => {}
        }
    }
    "NOT_FOUND"
}

fn main() {
    let mut t_line = String::new();
    io::stdin().read_line(&mut t_line).expect("failed to read line");
    let t: i64 = t_line.trim().parse().expect("invalid input");

    for _ in 0..t {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read line");
        println!("{}", find_first_vowel(line.trim()));
    }
}