use leetcode_88_lib::Solution;

fn main() {
    let mut nums1 = vec!(0,0,3,0,0,0,0,0,0);
    let mut nums2 = vec!(-1,1,1,1,2,3);
    Solution::merge(&mut nums1, 3, &mut nums2, 6);
    print!("{:?}", nums1);
}




