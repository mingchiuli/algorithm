
fn main() {
    let arr = vec![1,2,2,3,3,3,5,5,5,6,6,6,6,6,7];
    let num = Solution::majority_element(arr);
    print!("{}", num)
}

struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut num = nums[0];
        let mut count = 1;
        
        for i in 1..nums.len() {
            if nums[i] == num {
                count += 1;
            } else {
                count -= 1;
            }
            if count == 0 {
                num = nums[i];
                count = 1;
            }
        }
        num
    }
}