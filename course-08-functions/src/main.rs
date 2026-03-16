fn convert(s: &str) -> i32 {
    s.parse().unwrap()
}

fn add_one(v: i32) -> i32 {
    if v == 0 {
        return 1;
    }
    v + 1
}

fn main() {
    let input = "1 4 60 34 27 3 33 67 87 14";

    let res: Vec<i32> = input.split(' ')
        .map(convert).map(add_one).map(|n| n * 2).collect();

    println!("{:?}", res);
}
