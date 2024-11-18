struct Solution;

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut nums = nums.clone();
        let mut index = 0;
        let mut count = 0;

        // Removing hills and valleys with more than one number
        while index < nums.len() - 1 {
            while index + 1 < nums.len() && nums[index] == nums[index + 1] {
                nums.remove(index + 1);
            }
            index = index + 1
        }

        println!("{:?}", nums);

        index = 1;

        for index in index..nums.len() - 1 {
            if nums[index - 1] < nums[index] && nums[index] > nums[index + 1]
                || nums[index - 1] > nums[index] && nums[index] < nums[index + 1]
            {
                count = count + 1
            }
        }

        count
    }
}

fn main() {
    println!(
        "{}",
        Solution::count_hill_valley(vec![
            57, 57, 57, 57, 57, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90,
            85, 85, 85, 86, 86, 86
        ])
    );
}
