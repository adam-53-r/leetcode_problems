struct Solution;

impl Solution {
    pub fn add_spaces(mut s:String, spaces: Vec<i32>) -> String {
        for index in 0..spaces.len() {
            s.insert((spaces[index] + index as i32) as usize, ' ');
        }

        s
    }
}


fn main() {
    println!("{}", Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec!(8,13,15)));
}
