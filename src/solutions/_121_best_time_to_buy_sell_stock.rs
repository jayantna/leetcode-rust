pub struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_b = prices[0];
        let mut profit = 0;
        for i in (0..prices.len()) {
            println!("{:?}", i);
            if prices[i] < min_b {
                min_b = prices[i];
            } else if prices[i] - min_b > profit {
                profit = prices[i] - min_b;
            }
            println!("iter {:?} {:?}", min_b, profit);
        }
        profit
    }

    pub fn run() {
        let prices = vec![1,2];
        let ans = Solution::max_profit(prices);
        println!("{:?}", ans);
    }
}
