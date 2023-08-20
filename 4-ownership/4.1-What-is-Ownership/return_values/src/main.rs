fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives(s2);

    println!("s2: {}", s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("your");

    some_string
}

fn takes_and_gives(a_string: String) -> String {
    a_string
}
