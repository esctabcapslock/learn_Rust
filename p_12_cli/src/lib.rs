
use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config{
    quary:String,
    filename:String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config,&'static str>{
        args.next();


        let quary = match args.next(){
            Some(arg) => arg,
            None=> return  Err("Didn't get a query string")
        };
        
        let filename = match args.next(){
            Some(arg) => arg,
            None=> return  Err("Didn't get a filename string")
        };


        Ok(Config {quary, filename})
    }
}


pub fn run(config:Config)->Result<(), Box<dyn Error>>{
    println!("quary:{}, lifetime:{}",config.quary, config.filename);

    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("text: {}",contents);
    Ok(())
}


mod test{
    use super::*;
    
    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

pub fn search<'a>(quary:&str,contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines() {
        // do something with line
        if line.contains(quary){
            results.push(line);
        }
    }
    results
}