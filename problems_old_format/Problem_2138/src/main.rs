struct Solution {}



impl Solution {
    pub fn divide_string(s: String, k: i32, fill: char) -> Vec<String> {
        
        let vec_len = (s.len() as f32 / k as f32).ceil() as usize;
        let mut result: Vec<String> = vec!["".to_string(); vec_len];
        let mut iter = s.chars().into_iter();

        for string in &mut result {

            for x in 0..k {
                match iter.next() {
                    Some(char) => string.push(char),
                    None => string.push(fill)
                }
            }            

        }

        return result;
    }
}



fn main() {
    println!("{:?}", Solution::divide_string("abcdefghi".to_string(), 3, 'x'));
}
