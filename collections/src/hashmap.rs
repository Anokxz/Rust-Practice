use std::collections::HashMap;
fn main(){
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    scores.insert("Green", 100);

    for (key, value) in &scores {
        println!("{key}, {value}");
    }

    let team_name = "Green";
    let team_score = scores.get(team_name).copied().unwrap_or(0);
    println!("The score of {team_name} is {team_score}");

    scores.entry("Green").or_insert(50); // Key exists so nothing happens
    scores.entry("Red").or_insert(11); //Generates a new value
    
    // or_insert method return a mutable reference of the value of the key
    let blue_teamscore = scores.entry("Blue").or_insert(0);
    *blue_teamscore += 1; // Updating the value in hashmap 

    println!("{scores:?}");

    let sentence = String::from("Hello World I am Hello am see you think was was");
    let mut word_count = HashMap::new();
    for word in sentence.split_ascii_whitespace(){
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{word_count:?}");
}