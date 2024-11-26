fn main() {
    let mut arr = vec![1,2,2,2,3,3,3,4,5];
    let out = Solution::remove_duplicates(&mut arr);
    println!("{:?}", out);
}


struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32
        }
        let mut fast = 2;
        let mut slow = 2;
        
        while fast < nums.len() {
            if nums[fast] != nums[slow - 2] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
    }
}