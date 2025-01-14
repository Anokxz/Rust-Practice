use std::io::{self, Write};

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    
    print!("Enter the first number : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).expect("Failed to read line"); 
    
    print!("Enter the second number : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    
    let a: i32 = a.trim().parse().expect("Invalid Input!!"); 
    let b: i32 = b.trim().parse().expect("Invalid Input!!");

    let mut max = a;
    if max < b {
        max = b;
    }
    println!("The maximum of {a} and {b} is {max}");
}
