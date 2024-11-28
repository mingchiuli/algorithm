use std::{cmp::max, i32::MAX};

fn main() {
    let arr = vec![7,1,5,3,6,4];
    let out = Solution::max_profit(arr);
    println!("{}", out);
}

struct Solution;


impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        
        let mut max_profit = 0;
        let mut min = MAX;
        
        for idx in 0..prices.len() {
            if prices[idx] < min {
                min = prices[idx];
            } else {
                max_profit = max(prices[idx] - min, max_profit);
            }
        }
    
        max_profit
    }
}