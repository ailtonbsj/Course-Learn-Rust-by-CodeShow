use std::io;

fn main() {
    let mut input = String::new();
    println!("Type a text: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading stdin");
    println!("{input}");
}
