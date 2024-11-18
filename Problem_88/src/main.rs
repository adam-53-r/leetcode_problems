use std::collections::VecDeque;


struct Solution {}

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        
        let mut out:Vec<i32> = Vec::new();

        for x in 0..nums1.len() as usize {
            if x > n {
                out.push(nums1[x]);
            }
            else if x > m {
                out.push(nums1[x]);
            }
            
            
            else if nums1[x] <= nums2[x]


        }

    }
}

fn main() {
    let mut nums1:Vec<i32> = vec![1,2,3,0,0,0];
    let mut nums2:Vec<i32> = vec![2,5,6];
    Solution::merge( &mut nums1, 3, &mut nums2, 3 );
    println!("{:?}", nums1);
}
