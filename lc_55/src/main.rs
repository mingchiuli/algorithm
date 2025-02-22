use std::cmp::max;

fn main() {
    let b = Solution::can_jump(vec![2,3,1,1,4]);
    println!(" = {}", b);
}


struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_reachable = 0;
        for (idx, ele) in nums.iter().enumerate() {
            if idx > max_reachable {
                return false;
            }
            max_reachable = max(max_reachable, idx + *ele as usize);
        }
        true
    }
}