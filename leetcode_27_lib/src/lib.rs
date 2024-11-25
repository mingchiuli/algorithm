

pub struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut idx = 0;
        for i in 0..nums.len() {
            if nums[i] != val {
                nums[idx] = nums[i];
                idx += 1;
            }
        }
        return idx as i32;
    }
}