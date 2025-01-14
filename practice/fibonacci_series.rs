use std::io::{self, Write};

fn main(){
    let mut n = String::new();
    print!("Enter the limit : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Error reading line");
    let n: i32 = n.trim().parse().expect("Invalid Input");

    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..n {
        let sum: u128 = a+b;
        print!("{sum} ");
        io::stdout().flush().unwrap();
        a = b;
        b = sum;
    }
}
