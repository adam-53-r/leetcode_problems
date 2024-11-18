struct Solution {}

impl Solution {
    pub fn find_non_min_or_max(nums: Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return -1;
        }
        
        let mut min = 0;
        let mut max = 0;
        for x in 1..nums.len() {
            if nums[x] < nums[min] {
                min = x
            }
            if nums[x] > nums[max] {
                max = x
            }
        }
        let mut out = 0;
        while out == min || out == max {
            out += 1;
        }
        return nums[out];
    }
}

fn main() {
    println!("{}", Solution::find_non_min_or_max(vec![2,4,25]));
}
