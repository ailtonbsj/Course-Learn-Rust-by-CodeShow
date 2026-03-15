use std::io;

fn main() {
    let mut input = String::new();
    println!("Type a text: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading stdin");

    input = input.trim().to_string();
    println!("{input}");
    println!("length of bytes: {}", input.len());
    println!("length: {}", input.chars().count());
}
