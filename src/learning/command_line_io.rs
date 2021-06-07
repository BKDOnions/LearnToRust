use std::env;
use std::fs;

pub fn get_args() {
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        println!("the {}th arg is {}", i, args[i]);
    }
}
pub fn get_file_content(){
    let content = fs::read_to_string()
}