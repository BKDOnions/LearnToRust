use crate::algorithms::solution::Solution;

mod algorithms;
mod implements;
mod learning;
mod leet_code;

fn main() {
    let str = String::from(", , , ,        a, eaefa");
    println!("{}", Solution::count_segments(str));
}
