use std::collections::HashSet;

use crate::algorithms::solution::Solution;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        let len = s.len();
        return match len < 10 {
            true => Vec::new(),
            false => {
                let mut container: Vec<String> = Vec::new();
                let mut set: HashSet<String> = HashSet::new();
                for idx in 10..len + 1 {
                    let cur = String::from(&s[idx - 10..idx]);
                    if !set.insert(String::from(&cur)) {
                        if !container.contains(&cur) {
                            container.push(cur);
                        }
                    }
                }
                container
            }
        };
    }
}
