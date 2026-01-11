use std::{cmp::max, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hashmap: HashMap<char, usize> = HashMap::new();
        let mut maxlen = 0;
        for i in 0..s.len() {
            for j in i..s.len() {
                let left = s.chars().nth(i).unwrap();
                let right = s.chars().nth(j).unwrap();
                let current_character = s.chars().nth(i).unwrap();
                if !hashmap.contains_key(&current_character) {
                    hashmap.insert(current_character, i);
                    let len = i-j;
                    maxlen = max(len, maxlen)
                }
            }
        }
        for (left, right) in (3..=10).enumerate(){
            println!("{} {}", left, right);
        }
        println!("{:?}", hashmap);
        println!("{}", maxlen);
        return 1;
    }

    pub fn run() {
        let s = String::from("fpwwkew");
        let ans = self::Solution::length_of_longest_substring(s);
        println!("{}", ans)
    }
}
