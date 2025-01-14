use std::io;

fn main(){
    println!("Temperature Convertion");

    println!("Enter the temperature to convert : ");
    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Error Reading Lines");
    
        println!("Enter 1 for Celsius to Fahrenheit\nEnter 2 for Fahrenheit to Celsius : ");
    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Error Reading Lines");

    let temp: f64 = temp.trim().parse().expect("Error converting into f64");
    let choice: i32 = choice.trim().parse().expect("Error converting into i32"); 

    if choice == 1 {
        println!("The Temperature is Fahrenheit is {}", fahrenheit(temp));
    } else if choice == 2 {
        println!("The Temperature is Celsius is {}", celsius(temp));
    } else {
        println!("Wrong Input Quiting");
    }
}


fn fahrenheit(temp: f64) -> f64 {
    temp * (9.0/5.0) + 32.0
}

fn celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}