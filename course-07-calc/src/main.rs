use std::io;
use std::io::Write;

fn main() {
    println!("{:-^40}", " My Calculator ");
    print!("Type your numbers: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading stdin");

    
    let numbers: Vec<i32> = input.split(",")
        .map(|c| c.trim().parse().expect("Error")).collect();
    let res: i32 = numbers.iter().sum();

    println!("Sum is {}", res);
}
