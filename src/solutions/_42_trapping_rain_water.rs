use std::cmp::max;

pub struct Solution;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut left_max = 0;
        let mut right_max = 0;
        let mut i = 0;
        let mut j = height.len()-1;
        while i<j {
            if height[i] < height[j] {
                if left_max > height[i] {
                    total += left_max-height[i];
                }
                else {
                    left_max = height[i];            
                }
                i+=1;
            }
            else {
                if right_max > height[j] {
                    total += right_max-height[j];
                }
                else {
                    right_max = height[j];
                }
                j-=1;
            }
        }
        return total;
    }

    pub fn run() {
        let height = vec![0,1,0,2,1,0,1,3,2,1,2,1];
        let ans = Solution::trap(height);
        println!("{:?}", ans);
    }
}