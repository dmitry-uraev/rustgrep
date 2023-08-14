extern crate rustgrep;

use std::env;
use std::process;
use rustgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem when parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rustgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
