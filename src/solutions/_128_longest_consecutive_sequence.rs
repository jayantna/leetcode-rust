use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn _128_longest_consecutive_sequence(nums: Vec<i32>) -> i32 {
        let mut hashset: HashSet<_> = nums.iter().collect();
        hashset
            .iter()
            .filter(|&&x| !hashset.contains(&(x - 1)))
            .map(|&&x| {
                (x..).take_while(|x| hashset.contains(x)).count()
            }).max().unwrap_or(0) as _
    }
    pub fn run() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        let result = Solution::_128_longest_consecutive_sequence(nums);
        println!("{:?}", result);
    }
}
