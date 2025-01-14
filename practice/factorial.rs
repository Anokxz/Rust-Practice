use std::io::{self, Write};

fn main(){
    // Getting the input n
    let mut n = String::new();
    print!("Enter the number : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Error reading line");
    let n: i32 = n.trim().parse().expect("Invalid Input");

    println!("The factorial of {n} is {}", factorial(n));
}

fn factorial(num: i32) -> i32{
    if num <= 1 {
      return 1
    }
    return num * factorial(num - 1);
  }
