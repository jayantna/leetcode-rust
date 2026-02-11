pub struct Solution;
impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut start = 1;
        let mut end = piles.iter().copied().max().unwrap();
        fn condition(mid: i32, piles: &Vec<i32>, h: i32) -> bool {
            let time: i64 = piles.iter().fold(0, |acc, &x| {
                // Calculation: ceil(x / mid) using integer arithmetic
                // Cast to i64 to prevent overflow during addition
                let hours = (x as i64 + mid as i64 - 1) / mid as i64;
                acc + hours
            });
            time <= h as i64
        }
        while start < end {
            let mid = start + (end - start) / 2;
            if condition(mid, &piles, h) {
                end = mid
            } else {
                start = mid + 1
            }
        }
        start
    }

    pub fn run() {
        let piles = vec![3, 6, 7, 11];
        let h = 8;
        let result = Solution::min_eating_speed(piles, h);
        println!("{:?}", result);
    }
}
