fn main() {
    let my_int: i32 = 101;
    let my_float: f64 = 3.1415;
    let my_bool: bool = true;
    let my_char: char = '✅';
    let my_tuple: (i32, f64, bool, char) = (102, 2.71, false, '🦀');
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    let my_hex: u32 = 0xff;
    let my_oct: u32 = 0o77;
    let my_bin: u32 = 0b1010_1010;

    println!("{:?}", my_int);
    println!("{:?}", my_float);
    println!("{:?}", my_bool);
    println!("{:?}", my_char);
    println!("{:?}", my_tuple);
    println!("{:?}", my_tuple.3);
    println!("{:?}", my_array);
    println!("{:?}", my_array[3]);
    println!("{:?}", &my_array[1..4]);
    println!("{:?}", my_hex);
    println!("{:?}", my_oct);
    println!("{:?}", my_bin);
}
