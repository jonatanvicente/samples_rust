use std::{env, process};
use minigrep_tdd::Config;

#[allow(unused_variables)]

/*
    cargo run -- frog poem.txt
    cargo run -- body poem.txt
    cargo run -- monomorphization poem.txt
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });
    
    
    
    if let Err(e) = minigrep_tdd::run(config) {
        process::exit(1);
    }
    
}

