use std::io;
use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    println!("Number Guessing Game");
    
    let secert_number = rand::thread_rng().gen_range(1..=100);
    let mut current_guess = 1;
    let total_guess = 5;
    loop {
        if current_guess > total_guess {
            println!("Out of guesses!! :( ");
            break;
        }
        println!("Please input your Number : ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed : {}", guess);

        //Method 1 (Using Crates)
        // match guess.cmp(&secert_number)
        // {
        //     Ordering::Less => println!("Too small"),
        //     Ordering::Greater => println!("Too big"),
        //     Ordering::Equal => {
        //         println!("You win");
        //         break;
        //     },
        // }

        // Method Two (Using Control FLow)
        if guess == secert_number {
            println!("You Won");
            break
        } else if guess < secert_number {
            println!("Too small");
        } else {
            println!("Too Big");
        }
        current_guess += 1;
    }
}
