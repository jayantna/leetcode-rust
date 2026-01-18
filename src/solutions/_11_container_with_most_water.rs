use std::cmp::{max, min};
pub struct Solution;
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut water = 0;
        let mut left = 0;
        let mut right = height.len()-1;
        while left < right {
            let min_h = min(height[left], height[right]);
            let wide: i32 = (right - left).try_into().unwrap();
            water = water.max(wide * min_h);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }
        water as i32
    }
    pub fn run() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = Solution::max_area(height);
        println!("{:?}", result);
    }
}
