use std::result;

struct Solution {}



impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {

        let mut result = 0;
        let mut x = num1;
        let mut y = num2;
        
        while x != 0 && y != 0 {
            if x < y {
                y = y - x
            }
            else {
                x = x - y
            }
            result = result + 1;
        }


        return result;
    }
}



fn main() {
    println!("{}", Solution::count_operations(10, 10));
}
