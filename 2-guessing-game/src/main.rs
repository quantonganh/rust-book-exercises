use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the salary!");

    let start = 1000;
    let end = 10000;
    let step = 100;

    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range({ start }..={ end });

    let secret_number = (random_number - start) / step * step + start;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
