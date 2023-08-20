fn five() -> i32 {
    5;
}

fn main() {
    // let x = (let y = 6);

    // let y = {
    //     let x = 3;
    //     x + 1
    // };

    let x = five();

    println!("The value of x is: {x}");
}
