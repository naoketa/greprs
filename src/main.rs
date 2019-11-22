use std::io::Read;
use std::env;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something wrong with reading the file");
    
    let results = search(&config.query, &contents);
    
    for line in results {
        println!("{}", line);
    }
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Config {query, filename}
    }
}
