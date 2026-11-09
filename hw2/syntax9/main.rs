use std::io;

fn analyze_file(name: &str, size: u32) {
    let ext = match name.rfind('.') {
        Some(idx) => &name[idx + 1..],
        None => "",
    };

    let size_mb = size as f64 / 1024.0;

    match ext {
        "rs" => println!("Rust Source File, Size: {:.1} MB", size_mb),
        "txt" => println!("Text Document, Size: {:.1} MB", size_mb),
        "json" => println!("JSON Data File, Size: {:.1} MB", size_mb),
        _ => println!("Unknown File Type"),
    }
}

fn main() {
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("failed to read line");
        let name = line.trim();

        if name == "STOP" {
            break;
        }

        let mut size_line = String::new();
        io::stdin().read_line(&mut size_line).expect("failed to read line");
        let size: u32 = size_line.trim().parse().expect("invalid input");

        if name.starts_with('.') {
            continue;
        }

        analyze_file(name, size);
    }
}