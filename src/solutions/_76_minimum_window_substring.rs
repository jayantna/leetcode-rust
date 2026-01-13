use std::{
    collections::{HashMap, VecDeque},
    result,
};

pub struct Solution;

impl Solution {
    pub fn _76_minimum_window_substring<'a>(s: &'a str, t: &'a str) -> &'a str {
        //Hashmap for t
        let mut target_counter: HashMap<char, isize> = HashMap::new();
        t.chars()
            .for_each(|x| *target_counter.entry(x).or_default() += 1);
        let mut left = 0;
        let mut result_string = String::new();
        // Hashmap for s
        let mut counter: HashMap<char, isize> = HashMap::new();

        for (right, c) in s.chars().enumerate() {
            if target_counter.contains_key(&c) {
                *counter.entry(c).or_default() += 1;
                println!("{:?}", counter);
            }
        }
        println!("{}", result_string);
        ""
    }
    pub fn run() {
        let s = "ASDCVSDFBANC";
        let t = "ABC";
        let result = Solution::_76_minimum_window_substring(s, t);
        println!("Minimum window: {}", result);
    }
}
