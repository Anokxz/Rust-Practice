use std::io::{self, Write};

fn main(){
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    print!("Enter the length of side one : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut a).expect("Error reading line");
    let a: i32 = a.trim().parse().expect("Invalid Input");

    print!("Enter the length of side two : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut b).expect("Error reading line");
    let b: i32 = b.trim().parse().expect("Invalid Input");

    print!("Enter the length of side three : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut c).expect("Error reading line");
    let c: i32 = c.trim().parse().expect("Invalid Input");

    if a+b > c && a+c > b && b+c > a {
	println!("Its a valid triangle.");
    } else {
	println!("Its not a valid triangle.");
    }

}
