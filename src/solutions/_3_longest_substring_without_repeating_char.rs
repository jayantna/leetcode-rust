use std::{
    cmp::max,
    collections::{HashMap, HashSet, hash_map::OccupiedEntry},
};

pub struct Solution;
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut longest = 0;
        let mut left = 0;
        let mut set: HashSet<char> = HashSet::new();
        let chars: Vec<_> = s.chars().collect();

        for (right, &c) in chars.iter().enumerate() {
            while set.contains(&c) {
                set.remove(&chars[left]);
                left += 1;
            }
            set.insert(c);
            longest = max(longest, right - left + 1);
        }
        longest as i32
    }

    pub fn run() {
        let s = String::from("pwwkew");
        let ans = Solution::length_of_longest_substring(s);
        println!("{:?}", ans);
    }
}
