use std::collections::HashMap;

use crate::algorithms::solution::Solution;

impl Solution {
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let path_map: HashMap<String, String> = paths.iter().map(|path| (path[0].clone(), path[1].clone())).collect();
        let mut next_dest = paths[0][1].clone();
        loop {
            if path_map.contains_key(&next_dest) {
                next_dest = path_map.get(&next_dest).unwrap().clone();
            } else {
                break;
            }
        }
        next_dest
    }
}