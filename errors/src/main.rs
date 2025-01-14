use std::fs::{File, read_to_string};
use std::io::{self, ErrorKind};

#[allow(unused)]
fn main() {
    // let file = File::open("Hello.txt");
    
    // let file = match file {
    //     Ok(file_handle) => file_handle,
    //     Err(error) => {
    //         match error.kind() {
    //             ErrorKind::NotFound => match File::create("Hello.txt") {
    //                 Ok(file) => file,
    //                 Err(error) => panic!("Problem creating the file: {error}")
    //             },
    //             other_error => panic!("Problem opening the file: {other_error}"),
    //         }    
    //     },
    // };

    // let username =  get_username_from_file().unwrap_or_else(|error| {
    //     panic!("Error : {}",error);
    // });

    another_main()

}

fn get_username_from_file() -> Result<String, io::Error> {
    read_to_string("file.txt")
}


struct Guess {
    number: u32,
}

impl Guess {
    fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Value should be in between 1 and 100, got the value {value}.");
        }

        Guess { number: value }
    }

    fn value(&self) -> u32 {
        self.number
    }
}

fn another_main() {
    let guess = 3;

    let test = Guess::new(guess);
    println!("The value is {}", test.value());
}
