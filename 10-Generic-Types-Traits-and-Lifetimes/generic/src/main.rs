fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];

    let result = largest(&nums);
    println!("The largest number is {}", result);

    let chars = vec!['a', 'b', 'c', 'd', 'e'];

    let result = largest(&chars);
    println!("The largest char is {}", result);

    let p = Point { x: 4, y: 5 };
    println!("p.x = {}", p.x());
}
