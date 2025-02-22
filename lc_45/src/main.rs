use std::cmp::max;

fn main() {
    let solution = Solution::jump(vec![1,3,1,1,1,1]);
    println!("{}", solution);
}

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut end = 0;
        let mut max_position = 0;
        let mut step = 0;
        
        for i in 0..nums.len() - 1 {
            max_position = max(max_position, i + nums[i] as usize);
            if max_position >= nums.len() - 1 {
                return step + 1;
            }
            if i == end {
                end = max_position;
                step += 1;
            }
        }
        return step;
    }
}