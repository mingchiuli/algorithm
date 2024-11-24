


fn main() {
    let mut nums1 = vec!(0,0,3,0,0,0,0,0,0);
    let mut nums2 = vec!(-1,1,1,1,2,3);
    Solution::merge(&mut nums1, 3, &mut nums2, 6);
    print!("{:?}", nums1); 
    
}

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if n == 0 {
           return;   
        }
        
        if m == 0 {
            nums1.clear();
            for ele in nums2 {
                nums1.push(*ele);
            }
            return;
        }
        
        let mut m = m as usize;
        let mut n = n as usize;
        
        let mut idx = nums1.len();
        
        loop {
            let i1 = nums1[m - 1];
            let i2 = nums2[n - 1];
            if i1 > i2 {
                nums1[idx - 1] = i1;
                nums1[m - 1] = 0;
                m -= 1;
            } else {
                nums1[idx - 1] = i2;
                nums2[n - 1] = 0;
                n -= 1;
            }
            idx -= 1;
            if m == 0 || n == 0 {
                break;
            }
        }
        
        if n > 0 {
            for i in 0..=n - 1 {
                nums1[i] = nums2[i];
            }
        }
    }
}


