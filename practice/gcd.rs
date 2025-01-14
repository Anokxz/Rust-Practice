use std::io::{self, Write};

fn main(){
    let mut num1 = String::new();
    print!("Enter the number one : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num1).expect("Error reading line");
    let num1: i32 = num1.trim().parse().expect("Invalid Input");

    let mut num2 = String::new();
    print!("Enter the number one : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num2).expect("Error reading line");
    let num2: i32 = num2.trim().parse().expect("Invalid Input");

    println!("The GCD of {num1} and {num2} is {}", gcd(num1, num2));
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
	return a;
    } else {
	return gcd(b, a%b)
    }
}
