fn main() {
    let mut arr = vec!(1, 1, 2, 4, 5, 6, 6);
    let out = Solution::remove_duplicates(&mut arr);
    println!("{}", out);
}

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut idx = 1;
        for i in 1..nums.len() {
            if nums[i] != nums[i - 1] {
                nums[idx] = nums[i];
                idx += 1
            }
        }
        idx as i32
    }
}
