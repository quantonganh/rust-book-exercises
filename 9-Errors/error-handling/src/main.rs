use std::error::Error;
use std::{fs::File, io::ErrorKind};

fn main() -> Result<(), Box<dyn Error>> {
    let f_result = File::open("hello.txt");
    let f = match f_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Failed to create file: {:?}", e),
            },
            other_error => {
                panic!("Failed to open file: {:?}", other_error);
            }
        },
    };

    let g = File::open("goodbye.txt")?;

    Ok(())
}
