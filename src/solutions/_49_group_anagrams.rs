use std::collections::HashMap;

use rand::seq::index;

pub struct Solution;
impl Solution {
    pub fn _49_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();
        let offset = b'a' as usize;
        let index = 0;
        for words in strs {
            let mut chars: [u8; 26] = [0; 26];
            let word_bytes: Vec<u8> = words.bytes().collect();
            for bytes in word_bytes {
                chars[bytes as usize-offset]=1;
            }
            map.entry(chars).or_default().push(words);
        }
        map.values().cloned().collect::<Vec<Vec<String>>>()
    }

    pub fn run() {
        let s: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();

        let result = &self::Solution::_49_group_anagrams(s);
        println!("{:?}", result);
    }
}
