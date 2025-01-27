struct Solution {}

impl Solution {
    pub fn find_closest_number(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        nums.iter().for_each(|x| {
            if x.abs() < result.abs() {
                result = *x;
            }
            else if x.abs() == result.abs() {
                result = result.max(*x);
            }
        });
        result
    }
}


fn main() {
    println!("{}", Solution::find_closest_number([-4,-2,4,8].to_vec()));
}
