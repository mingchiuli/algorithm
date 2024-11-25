use leetcode_27_lib::Solution;

fn main() {
    let mut nums = vec!(3,2,2,3);
    let res = Solution::remove_element(&mut nums, 3);
    println!("{}", res)
}
