fn main() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // let s = format!("{s1}-{s2}-{s3}");
    let s = s1 + s2 + s3;

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    // let s = &hello[0..1];
    println!("{s}");

    for c in "xin chào".chars() {
        println!("{c}");
    }

    for c in "xin chào".bytes() {
        println!("{c}");
    }
}
