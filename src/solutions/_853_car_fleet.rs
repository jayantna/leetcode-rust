// Minstack
use itertools::Itertools;

pub struct Solution;
impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let sorted_time: Vec<f32> = position
            .into_iter()
            .zip(speed)
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
            .map(|x| (target - x.0) as f32 / x.1 as f32)
            .collect();
        let mut stack: Vec<f32> = vec![];
        println!("{:?}", sorted_time);
        for i in sorted_time {
            if stack.len() == 0 {
                stack.push(i);
            }
            while stack.len() != 0 && stack.last().copied().unwrap() <= i {
                stack.pop();
            }
            stack.push(i);
        }
        println!("{:?}", stack);
        0
    }
    pub fn run() {
        let target = 10;
        let position = vec![6,8];
        let speed = vec![3,2];
        let result = Solution::car_fleet(target, position, speed);
        println!("{:?}", result);
    }
}
