use std::collections::{HashMap, btree_map::OccupiedEntry};

pub struct Solution;

impl Solution {
    pub fn _340_longest_substring_with_at_most_k_distint_characters(s: &str, k: usize) -> usize {
        let mut counter: HashMap<char, usize> = HashMap::new();
        let mut left = 0;
        let mut longest = 0;
        for (right, c) in s.chars().enumerate() {
            *counter.entry(c).or_insert(0) += 1;
            while counter.len() > k {
                let char_left = s.chars().nth(left).unwrap();
                counter.entry(char_left).and_modify(|x| *x -= 1);
                if counter[&char_left] == 0{
                    counter.remove(&char_left);
                }
                left += 1;
            }
            println!("{:?}", counter);
            longest = longest.max(right-left+1)
        }
        return longest;
    }
    pub fn run() {
        let s = "eceba";
        let k = 2;
        let result =
            &self::Solution::_340_longest_substring_with_at_most_k_distint_characters(s, k);
        println!("{}", result);
    }
}
