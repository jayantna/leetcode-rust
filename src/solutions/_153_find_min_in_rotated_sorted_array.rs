pub struct Solution;
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut start = 0;
        let mut end = nums.len() - 1;
        while start < end {
            let mid = start + (end-start)/2;
            if nums[mid] > nums[end] {
                start = mid +1;
            }
            else{
                end = mid
            }
        }
        nums[start]
    }
    pub fn run() {
        let nums = vec![3, 4, 5, 6, 1, 2];
        let result = Solution::find_min(nums);
        println!("{:?}", result);
    }
}
