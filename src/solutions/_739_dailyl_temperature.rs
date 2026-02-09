pub struct Solution;
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(usize, usize)> = Vec::new();
        let mut result: Vec<i32> = vec![0; temperatures.len()];
        for (idx, i) in temperatures.into_iter().enumerate() {
            if stack.len() == 0 {
                stack.push((idx, i as usize))
            }
            while stack.len() != 0 && stack.last().unwrap().1 < i as usize && stack.len() > 0 {
                println!("{:?} {:?}", stack.last().unwrap(), (idx, i));
                let poped = stack.pop().unwrap();
                result[poped.0] = idx as i32 - poped.0 as i32;
            }
            stack.push((idx, i as usize));
        }
        println!("{:?}", result);
        result
    }
    pub fn run() {
        let temperatures: Vec<i32> = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let result = Solution::daily_temperatures(temperatures);
        println!("{:?}", result);
    }
}
