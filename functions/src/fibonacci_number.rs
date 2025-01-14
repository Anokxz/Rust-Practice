use std::io;
fn main(){
    println!("Enter the range to print : ");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Error reading line");
    let number: u32 = number.trim().parse().expect("Error Converting into u32");

    let mut a = 0;
    let mut b = 1;

    for _ in 1..number+1 {
        println!("{}", a);
        
        let sum = a + b;
        a = b;
        b = sum;
    }
}