use std::{collections::HashMap, io};

fn main() {
    let mut department: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut command  = String::new();
        println!("Enter the commands");
        io::stdin()
            .read_line(&mut command)
            .expect("Error reading lines");

        command = command.trim().to_lowercase();
        let command: Vec<&str> = command.split_whitespace().collect();

        // Handle empty input or invalid command length
        if command.is_empty() {
            println!("Invalid command. Please enter a valid command.");
            continue;
        }
        // println!("The command is {}", command_list);
        if command[0] == "print" {
            if command.len() != 2 {
                println!("Invalid Command: Usage -> print [deparement] or print all");
            } else if department.is_empty() {
                println!("The List is Empty");
            } else {
                if command[1] == "all" {
                    for (key, value) in &department {
                        if value.is_empty() {

                        }  else {
                            println!("The Department is {key} : ");
                            for people_name in value {
                                println!("{people_name}");
                            }
                        }
                    }
                } else {
                    let deparment_name = command[1];
                    

                    if let Some(people_name) = department.get(deparment_name) {
                        println!("The People in the {} department are : ", deparment_name);
                        for person in people_name {
                            println!("{}", person);
                        }
                    } else {
                        println!("The Department is not present");
                    }
                }

            }
        } else if command[0] == "add" {
            if command.len() != 4 || command[2] != "to" {
                println!("Invalid Command Usage: add [people_name] to [deparment]");
            } else {
                let people_name = String::from(command[1]);
                let deparment_name = String::from(command[3]);

                let list = department.entry(deparment_name.clone()).or_insert(Vec::new());
                list.push(people_name.clone());

                println!("{} added to the {} department.", people_name, deparment_name);
            }
        } else if command[0] == "quit" {
            println!("Existing Program");
            break;
        }
        else {
            println!("Invalid command Usage: add, print, quit");
        }
    }
}