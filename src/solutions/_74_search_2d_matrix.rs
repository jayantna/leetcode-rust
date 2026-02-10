use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let flat: Vec<i32> = matrix.into_iter().flat_map(|x| x).collect();
        let mut start = 0;
        let mut end = flat.len();
        while start < end {
            let mid = start + (end - start) / 2;
            match flat[mid].cmp(&target) {
                Ordering::Equal => return true,
                Ordering::Greater => end = mid,
                Ordering::Less => start = mid + 1,
            };
        }
        false
    }
    pub fn run() {
        let matrix: Vec<Vec<i32>> = [
            [1, 3, 5, 7].into(),
            [10, 11, 16, 20].into(),
            [23, 30, 34, 60].into(),
        ]
        .into();
        let target = 3;
        let result = Solution::search_matrix(matrix, target);
        println!("{:?}", result);
    }
}
