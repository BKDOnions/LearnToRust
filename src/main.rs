use leet_code::no211;

use crate::leet_code::no211::WordDictionary;

mod algorithms;
mod implements;
mod learning;
mod leet_code;

fn main() {
    let mut dict = WordDictionary::new();
    dict.add_word(String::from("bad"));
    dict.add_word(String::from("dad"));
    dict.add_word(String::from("mad"));
    println!("{}", dict.search(String::from("b..")));
}
