use std::error::Error;
use std::fs;
use std::env;


pub struct Config {
    pub query: String,
    pub filepath: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result<Config, &'static str> {

        args.next();
        //Error handling
        // if args.len() < 3 {
        //     return Err("not enough arugments")
        // }

        let query = {
            if let Some(query) = args.next() {
                query
            } else {
                return Err("Didn't get a query string")
            }
        };

        let filepath = {
            if let Some(filepath) = args.next() {
                filepath
            } else {
                return Err("Didn't get a filepath string")
            }
        };
    
        //Enviroment Variables
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config {query, filepath, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let content = fs::read_to_string(config.filepath)?;
    // println!("The file contains : {}", content);

    let results = {
        if config.ignore_case {
            search_case_insensitive(&config.query, &content)
        } else {
            search(&config.query, &content)
        }
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results= Vec::new();
    // for line in content.lines() {
    //     if line.contains(query) {
    //         results.push(line)
    //     }
    // }
    
    // results

    //Using Iterators 
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut results= Vec::new();

    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
        
    } 

    #[test]
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
        
    }
}