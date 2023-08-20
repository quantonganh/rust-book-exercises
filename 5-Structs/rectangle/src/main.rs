#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let r = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&r);

    println!("area: {}", area(&r));
}

fn area(r: &Rectangle) -> u32 {
    r.width * r.height
}
