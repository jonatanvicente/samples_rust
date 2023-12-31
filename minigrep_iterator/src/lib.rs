use std::error::Error;
use std::{env, fs};

pub struct Config {
	  pub query: String,
	  pub file_path: String,
	  pub ignore_case: bool,
}

#[allow(unused_variables)]
impl Config {
	  // args can be any type that implements the Iterator type and returns String items
	  pub fn build(
			mut args: impl Iterator<Item = String>,
	  ) -> Result<Config, &'static str> {
			
			args.next(); // skip the first value (program name)
			
			/*
			   If next returns a Some, we use a match to extract the value.
			   If it returns None, it means not enough arguments were given and we return early with an Err value.
			 */
			let query = match args.next() {
				  Some(arg) => arg,
				  None => return Err("Didn't get a query string"),
			};
			
			let file_path = match args.next() {
				  Some(arg) => arg,
				  None => return Err("Didn't get a file path"),
			};
			
			let ignore_case = env::var("IGNORE_CASE").is_ok();
	
			Ok(Config {
				  query,
				  file_path,
				  ignore_case,
			})
	  }
}

#[allow(unused_variables)]
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	  let contents = fs::read_to_string(config.file_path)?;
	  
	  let results = if config.ignore_case {
			search_case_insensitive(&config.query, &contents)
	  } else {
			search(&config.query, &contents)
	  };
	  
	  for line in results {
			println!("{line}");
	  }
	  Ok(())
}

#[allow(unused_variables)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	  contents
		  .lines()
		  .filter(|line| line.contains(query))// filter para mantener sonla las line.contains(query) cuando retorna true
		  .collect()//pasamos a un nuevo vector
}


pub fn search_case_insensitive<'a>(
	  query: &str,
	  contents: &'a str,
) -> Vec<&'a str> {
	  
	  let query = query.to_lowercase();
	  
	  contents
		  .lines()
		  .filter(|line| line.to_lowercase().contains(&query))// filter para mantener sonla las line.contains(query) cuando retorna true
		  .collect()
}



#[cfg(test)]
mod tests {
	  use super::*;
	  
	  #[test]
	  fn case_sensitive() {
			let query = "duct";
			let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
			
			assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	  }
	  
	  #[test]
	  fn case_insensitive() {
			let query = "rUsT";
			let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
			
			assert_eq!(
				  vec!["Rust:", "Trust me."],
				  search_case_insensitive(query, contents)
			);
	  }
}


