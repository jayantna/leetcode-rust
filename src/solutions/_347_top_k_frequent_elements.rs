use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn _347_top_k_frequent_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counter: HashMap<_, _> = nums.iter().fold(HashMap::new(), |mut acc, num| {
            acc.entry(num.clone()).and_modify(|x| *x += 1).or_insert(1);
            acc
        });
        let mut ar: Vec<(i32, i32)> = counter.into_iter().collect();
        ar.sort_by(|a, b| b.1.cmp(&a.1));
        ar.iter().take(k as usize).map(|k| k.0).collect()
    }
    pub fn run() {
        let nums = vec![1, 2, 1, 2, 1, 2, 3, 1, 3, 2];
        let k = 2;
        let result = &self::Solution::_347_top_k_frequent_elements(nums, k);
        println!("{:?}", result);
    }
}
