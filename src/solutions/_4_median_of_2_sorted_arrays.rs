use itertools::Itertools;

pub struct Solution;
impl Solution {


    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        
        0.0
    }
    // Merge and sort 3ms
    // pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    //     let flat: Vec<i32> = [nums1, nums2]
    //         .into_iter()
    //         .flat_map(|x| x)
    //         .sorted()
    //         .collect();
    //     println!("{:?}", flat);
    //     let len = flat.len();
    //     if len % 2 != 0 {
    //         return flat[len / 2] as f64;
    //     } else {
    //         return (flat[(len / 2)-1] + flat[len / 2]) as f64 / 2 as f64;
    //     }
    // }
    pub fn run() {
        let nums1 = vec![1];
        let nums2 = vec![];
        let result = Solution::find_median_sorted_arrays(nums1, nums2);
        println!("{:?}", result);
    }
}
