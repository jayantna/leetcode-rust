use std::collections::HashMap;


pub struct Solution;

impl Solution {
    pub fn _242_valid_anagram(s: String, t: String)-> bool {
        let mut map = HashMap::<char, isize>::new();
        s.chars().for_each(|x| {*map.entry(x).and_modify(|y| *y+=1).or_insert(1);});
        t.chars().for_each(|x| {*map.entry(x).and_modify(|y| *y-=1).or_insert(-1);});
        return map.into_values().all(|x| x==0);
    }
    pub fn run() {
        let s: String = String::from("anagram");
        let t: String = String::from("nagaram");
        let result = &self::Solution::_242_valid_anagram(s, t);
        println!("{result}");
    }

}