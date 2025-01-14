use std::io;
fn main(){
    let mut word = String::new();

    println!("Enter the Word: ");
    io::stdin()
        .read_line(&mut word)
        .expect("Error Reading Lines");

    let mut word = String::from(word.trim());

    let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

    if let Some(first_letter) = word.chars().next() {
        word = if vowels.contains(&first_letter) {
            format!("{word}-hay")
        } else {
            format!("{}-{}ay", &word[1..], first_letter)
        };
        println!("The pig lain format is {word}");
    } else {
        println!("The String is Empty");
    } 
}