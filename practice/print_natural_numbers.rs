use std::io::{self, Write};

fn main(){
    
    let mut n = String::new();
    print!("Enter the limit : ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: i32 =  n.trim().parse().expect("Invaild Input!!");

    for i in 1..n+1 { 
        print!("{i} ");
        io::stdout().flush().unwrap();
    }

}
