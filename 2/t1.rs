use std::io;

fn analyze_file(name: &str, size: u32) {
    let extension = match name.rfind('.') {
        Some(index) => &name[index + 1..],
        None => "",
    };

    let size_mb = size as f64 / 1024.0;

    match extension {
        "json" => println!("JSON Data File, Size: {:.1} MB", size_mb),
        "txt" => println!("Text Document, Size: {:.1} MB", size_mb),
        "rs" => println!("Rust Source File, Size: {:.1} MB", size_mb),
        _ => println!("Unknown File Type"),
    }
}

fn main() {
    let mut input = String::new();

    loop {
        input.clear();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let name = input.trim();

        if name == "STOP" {
            break;
        }

        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let size: u32 = input.trim().parse().unwrap();

        if name.starts_with('.') {
            continue;
        }

        analyze_file(name, size);
    }
}