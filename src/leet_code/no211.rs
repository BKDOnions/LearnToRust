use std::collections::{HashMap, HashSet};

pub struct WordDictionary {
    dict: HashMap<usize, HashSet<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    pub fn new() -> Self {
        WordDictionary {
            dict: Default::default(),
        }
    }

    pub fn add_word(&mut self, word: String) {
        &self.dict.entry(word.len()).or_default().insert(word);
    }

    pub fn search(&self, word: String) -> bool {
        return match &self.dict.get(&word.len()) {
            None => false,
            Some(set) => {
                if set.contains(&word) {
                    return true;
                }
                let mut words_map: HashMap<usize, char> = HashMap::new();
                for (idx, char) in word.chars().enumerate() {
                    if char != '.' {
                        words_map.insert(idx, char);
                    }
                }
                if words_map.is_empty() {
                    return true;
                }
                for member in set.iter() {
                    let mut res = true;
                    for word in words_map.iter() {
                        if !member.chars().nth(*word.0).eq(&Some(*word.1)) {
                            res = false;
                            break;
                        }
                    }
                    if res {
                        return res;
                    }
                }
                false
            }
        };
    }
}
/**
 * Your WordDictionary object will be instantiated and called as such:
 * let obj = WordDictionary::new();
 * obj.add_word(word);
 * let ret_2: bool = obj.search(word);
 */