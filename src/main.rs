use crate::algorithms::solution::Solution;

mod algorithms;
mod implements;
mod learning;
mod leet_code;

fn main() {
    let s = "AAAAAAAAAAA";
    println!(
        "{:?}",
        Solution::find_repeated_dna_sequences(String::from(s))
    );
}
