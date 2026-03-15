fn main() {
    println!("{:=^40}", " My App ");
    println!("{}", "Welcome !".to_uppercase().replace("!", "?"));
    let multilines = "\n\
        How are you?\n\
        R U ok?\n\
        ";
    println!("{}", multilines);
    println!("{}", "-".repeat(40));
}
