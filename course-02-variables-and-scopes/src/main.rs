const MY_CONST: &str = "MY CONSTANT";

fn main() {
    let my_integer = 50;
    let my_float = my_integer as f64 + 0.1;
    let mut my_bool = true;
    
    println!("My vars: {my_float} {my_bool}");

    my_bool = false;
    println!("My multable var: {my_bool}");

    {
        let my_integer = my_integer + 50;
        println!("My inner var: {my_integer}");
    }
    println!("My inner var: {my_integer}");
    println!("{MY_CONST}");
}
