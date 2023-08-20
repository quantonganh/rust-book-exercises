fn main() {
    let s1 = String::from("hello");

    let len = length(&s1);

    println!("The length of '{}' is {}.", s1, len)
}

fn length(s: &String) -> usize {
    s.len()
}
