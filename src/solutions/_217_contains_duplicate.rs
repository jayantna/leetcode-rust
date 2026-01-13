use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn _217_contains_duplicate(nums: Vec<i32>) -> bool {
        let mut number_set: HashSet<i32> = HashSet::new();
        !nums.iter().all(|x| number_set.insert(*x))
    }
    pub fn run() {
        let nums: Vec<i32> = vec![1,2,3,4];
        let result = Solution::_217_contains_duplicate(nums);
        println!("{}", result);
    }
}
