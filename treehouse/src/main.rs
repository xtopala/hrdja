use std::io::stdin;

fn main() {
    println!("Hello traveler, what is your name?");
    let mut your_name = String::new();

    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read your name");

    println!("Hello, {}", your_name);
}
