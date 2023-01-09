use std::env;
use grep_clone::Config;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    println!("filename: {}", config.filename);
    println!("query: {}", config.query);

    grep_clone::run(config);
   
}

