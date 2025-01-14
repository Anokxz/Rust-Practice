fn main(){
    let s1 = String::from("Hello ");
    let s2 = String::from("Akshayan");

    
    let s4 = format!("{s1} {s2}");
    let s3 = s1 + &s2;
    println!("{s3}, {s4}");

    for letter in s3.chars(){
        print!("{letter}");
    }
}