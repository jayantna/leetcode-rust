use std::{cmp::Ordering, collections::HashMap};

pub struct Solution;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
                let mut i = 0;
        let mut j = numbers.len()-1;
        
        while i < j {
            match (numbers[i]+numbers[j]).cmp(&target) {
                Ordering::Less => i += 1,
                Ordering::Greater => j -= 1,
                Ordering::Equal => return vec![(i+1) as i32, (j+1) as i32],
            }
        }
        unreachable!();
    }

    pub fn run() {
    let numbers = vec![0,0,3,4];
    let target = 0;
    let result = Solution::two_sum(numbers, target);
    println!("{:?}", result);
    }
}