use std::collections::HashSet;


struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        nums.len() != HashSet::<i32>::from_iter(nums).len()
    }
}


fn main() {
    println!("{}", Solution::contains_duplicate(vec!(1,2,3,1)));
}
