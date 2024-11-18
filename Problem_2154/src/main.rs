struct Solution {}

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        
        let mut nums_sort = nums;
        nums_sort.sort();
        let mut x = original;

        for num in &nums_sort {

            if &x > nums_sort.last().unwrap() {
                break;
            }
            if num == &x {
                x = x*2;
            }
        }
        x
    }
}

fn main() {
    println!("{}", Solution::find_final_value(vec![2,7,9], 4));
}
