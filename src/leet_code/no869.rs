use crate::algorithms::solution::Solution;

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let dict: Vec<&str> = vec![
            "1",
            "2",
            "4",
            "8",
            "16",
            "23",
            "46",
            "128",
            "256",
            "125",
            "0124",
            "0248",
            "0469",
            "1289",
            "13468",
            "23678",
            "35566",
            "011237",
            "122446",
            "224588",
            "0145678",
            "0122579",
            "0134449",
            "0368888",
            "11266777",
            "23334455",
            "01466788",
            "112234778",
            "234455668",
            "012356789",
            "0112344778",
        ];
        let str_res = n.to_string();
        let mut byte_res = Vec::from(str_res.as_bytes());
        byte_res.sort();
        let res = String::from_utf8(Vec::from(byte_res)).unwrap();
        for element in dict.iter() {
            if element.eq(&res) {
                return true;
            }
        }
        false
    }
}
