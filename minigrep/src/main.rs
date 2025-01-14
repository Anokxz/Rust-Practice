use std::{env, process};
use minigrep::Config;
fn main() {
    // Handling arguments
    //let args: Vec<String> = env::args().collect();

    
    //Building the Configurations
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    //Printing the query to search.
    println!("The String to search is  : {}\n", config.query);

    //Running the Application
    if let Err(e) = minigrep::run(config) {
        println!("Appicaiton Error: {e}");
        process::exit(1);
    }
}



