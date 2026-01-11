use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_map = HashMap::new(); // A mapping to store numbers and their indices
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num; // Find the required number to reach the target
            if let Some(&index) = num_map.get(&complement) {
                return vec![index as i32, i as i32]; // Return indices of the complement and current number
            }
            num_map.insert(num, i); // Store the number with its index
        }
        vec![] // This line is never reached due to the problem guarantee
    }

    pub fn run() {
        let nums = vec![2,7,11,15];
        let target = 9;
        let ans = &self::Solution::two_sum(nums, target);
        println!("{:?}", ans);
    }
}
