pub struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut start = 0;
        let mut end = nums.len();
        while start < end {
            let mid = start + (end - start) / 2;
            if nums[mid] == target {
                return mid as i32;
            } else if nums[start] <= nums[mid] {
                if nums[start] <= target && target < nums[mid] {
                    end = mid - 1;
                } else {
                    start = mid + 1;
                }
            } else {
                if nums[mid] < target && target <= nums[end] {
                    start = mid + 1;
                } else {
                    end = mid - 1;
                }
            }
        }
        -1
    }
    pub fn run() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        let result = Solution::search(nums, target);
        println!("{:?}", result);
    }
}
