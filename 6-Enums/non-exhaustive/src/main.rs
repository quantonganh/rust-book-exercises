fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);

    match six {
        Some(x) => println!("{}", x),
        None => println!("No number"),
    }
}
