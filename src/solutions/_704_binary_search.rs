pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();
        while start < end {
            let mut mid = start + (end - start) / 2;
            if nums[mid] > target {
                end = mid;
            } else if nums[mid] < target {
                start = mid + 1;
            } else {
                return mid as i32;
            }
        }
        -1
    }
    pub fn run() {
        let nums = vec![1, 0, 3, 5, 9, 12];
        let target = 8;
        let result = Solution::search(nums, target);
        println!("{:?}", result);
    }
}
