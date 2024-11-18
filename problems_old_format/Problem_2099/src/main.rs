struct Solution {}



impl Solution {
    pub fn max_subsequence(nums: Vec<i32>, k: i32) -> Vec<i32> {

        if nums.len() == k as usize {
            return nums;
        }
        
        let mut result = nums;
        result.sort();
        return result[result.len() - k as usize..result.len()].to_vec();
    }
}


fn main() {
    println!("{:?}", Solution::max_subsequence(vec![2,1,3,3], 2));
}
