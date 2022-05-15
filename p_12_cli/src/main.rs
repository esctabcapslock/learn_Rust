extern crate p_12_cli;

use std::env;
use p_12_cli::*;


fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments err: {}",err);
        std::process::exit(1)
    });
    
    // println!("{:?}",args);
    run(config).unwrap();
}
