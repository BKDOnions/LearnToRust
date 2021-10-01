use crate::algorithms::solution::Solution;

mod algorithms;
mod implements;
mod learning;
mod leet_code;

fn main() {
    let paths = vec![vec!["B".to_string(), "C".to_string()], vec!["D".to_string(), "B".to_string()], vec!["C".to_string(), "A".to_string()]];
    println!("{}", Solution::dest_city(paths));
}
