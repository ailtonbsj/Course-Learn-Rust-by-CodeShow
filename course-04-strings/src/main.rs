static STATIC_VAR: &str = "My static string";

fn main() {
    println!("{STATIC_VAR}");
    let stack_var = "My stack string";
    println!("{stack_var}");
    let mut heap_var = String::new();
    heap_var.push('M');
    heap_var.push_str("y heap string");
    println!("{heap_var}");

    let mut my_string = String::from("My String 1");
    println!("{my_string}");

    my_string = "My String 2".to_string();
    println!("{my_string}");

    let my_array = ['M','y',' ','s','t','r','i','n','g',' ','3'];
    my_string = String::from_iter(my_array);
    println!("{my_string}");

    let my_string4: String = "My String 4".into();
    println!("{my_string4}");

    let my_string5 = "My String 5".to_owned();
    println!("{my_string5}");
}
