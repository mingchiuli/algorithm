
fn main() {
    let arr = vec![7,1,5,3,6,4];
    let out = Solution::h_index(arr);
    println!("{}", out);
}

struct Solution;


impl Solution {
    
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();
        for (i, ele) in citations.iter().rev().enumerate() {

            let gap = i as i32 + 1;
            if *ele < gap {
                return gap - 1;
            }
        }
        citations.len() as i32
    }
}