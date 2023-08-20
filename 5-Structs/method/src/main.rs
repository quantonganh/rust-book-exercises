#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("r1: {:#?}", r1);
    let r2 = Rectangle {
        width: 20,
        height: 40,
    };
    println!("r2: {:#?}", r2);
    let r3 = Rectangle {
        width: 40,
        height: 60,
    };
    println!("r3: {:#?}", r3);

    println!("area of r1: {}", r1.area());
    println!("Can r1 hold r2? {}", r1.can_hold(&r2));
    println!("Can r1 hold r3? {}", r1.can_hold(&r3));

    let sq = Rectangle::square(3);
    println!("sq: {:#?}", sq);
    println!("area of square: {}", sq.area());
    println!("area of square: {}", sq.area());
    println!("area of square: {}", sq.area());
}
