extern crate greprs;

use std::env;

use greprs::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    greprs::run(config);
}