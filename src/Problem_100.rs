struct Solution {}

impl Solution {
    pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec!(0,1)
        }
        let mut old_nums = nums.clone();
        nums.sort_unstable();
        
        let mut left_cursor = nums.iter().enumerate().peekable();
        let mut right_cursor = nums.iter().enumerate().rev().peekable();

        while left_cursor.clone().ne(right_cursor.clone()) {
            match left_cursor.peek().unwrap().1 + right_cursor.peek().unwrap().1 {
                x if x > target => {
                    right_cursor.next();
                },
                x if x < target => {
                    left_cursor.next();
                },
                _ => {
                    let x = old_nums.iter().position(|p| (*p).eq(&(*left_cursor.peek().unwrap().1 as i32))).unwrap() as i32;
                    return vec!(x,
                        old_nums.iter().position(|p| {
                            (*p).eq(&(*right_cursor.peek().unwrap().1 as i32))
                        }).unwrap() as i32
                    )
                }
            }
        }
        panic!("You fucked up!")
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec!(2,7,11,15), 9));
}
