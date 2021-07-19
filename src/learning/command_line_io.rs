use std::env;
use std::fs;

/// Config Struct
///
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
pub fn get_args() {
    // let args: Vec<String> = env::args().collect();
    // for i in 0..args.len() {
    // println!("the {}th arg is {}", i, args[i]);
    // }
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args, 2).unwrap();
    let content = fs::read_to_string(config.filename).expect("Do sth");
    println!("{}", content);
}

pub fn get_file_content() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[2];
    let content =
        fs::read_to_string(filename).expect("Something went wrong while opening the file");
    println!("Text of the file : \n{}", content);
}

impl Config {
    pub fn new(args: &[String], seq: u32) -> Result<Config, &'static str> {
        if seq <= 0 {
            return Err("Illegal Arguments");
        }
        if args.len() < (1 + seq * 2) as usize {
            return Err("not enough arguments");
        }

        let query = args[1 * seq as usize].clone();
        let filename = args[2 * seq as usize].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}