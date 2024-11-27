
fn main() {
    let mut arr = vec![1,2,3,4,5,6,7];
    Solution::rotate(&mut arr, 3);
    println!("{:?}", arr);
}


struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}