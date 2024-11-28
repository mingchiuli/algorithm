use std::cmp::max;


fn main() {
    let arr = vec![7,1,5,3,6,4];
    let out = Solution::max_profit(arr);
    println!("{}", out);
}

struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        let mut max_profit = 0;
        
        for idx in 1..prices.len() {
            max_profit = max(max_profit + prices[idx] - prices[idx - 1], max_profit);
        }
    
        max_profit
    }
}