use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("{:?}",args);
    println!("quary:{}, lifetime:{}",config.quary, config.filename);


    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");

    println!("text: {}",contents);

}

struct Config{
    quary:String,
    filename:String,
}

impl Config {
    fn new(args:&[String]) -> Config{
        if args.len() < 3 {
            panic!("not enough arguments")
        }

        let quary = args[1].clone();
        let filename = args[2].clone();
        Config {quary, filename}
    }
}
