use std::vec;

pub struct Solution;
impl Solution {
    pub fn _238_product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![1; nums.len()];
        let mut left_product = 1;
        let mut right_product = 1;
        ans = nums
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let val = left_product;
                left_product *= nums[i];
                return val;
            })
            .collect();
        for i in (0..nums.len()).rev() {
            ans[i] = ans[i] * right_product;
            right_product *= nums[i]
        }
        ans
    }
    pub fn run() {
        let nums = vec![1, 2, 3, 4];
        let result = &self::Solution::_238_product_except_self(nums);
        println!("{:?}", result);
    }
}
