fn say_hello(t: &str) {
    println!("Hello, {t}");
}

fn say_bye(t: &str) {
    println!("Bye, {t}");
}

fn say_hi(t: String) {
    println!("Hi, {t}");
}

fn say_nice(t: String) {
    println!("Nice, {t}");
}

fn say_nice_ref(t: &String) {
    println!("Nice, {t}");
}

fn to_upper(t: &mut String) {
    *t = t.to_uppercase();
}

fn add_prefix(t: &mut String) {
    *t = format!("PRE_{t}")
}

fn main() {
    // copy types are: i32, f64, bool, char, &str ...
    let a: i32 = 10;
    let b = &a;
    println!("Value of A is {}", a);
    println!("Value of B is {}", *b);

    // c variable has ownership of bytes "Text" in heap
    let c = String::from("Text");
    let d = c; // d now has ownership of bytes "Text"
    let f = &d; // f has borrowed ownership of "Text"
    println!("Value of C is {}", d);
    println!("Value of D is {}", f);

    let g = "My static text";
    say_hello(g);
    say_bye(g);

    let h = "My heap text".to_string();
    say_hi(h.clone());
    say_nice_ref(&h); // function has borrowed ownership o "My heap text"
    say_nice(h);

    let mut j = "On heap".to_string();
    to_upper(&mut j); // function has borrowed ownership
    add_prefix(&mut j);

}
