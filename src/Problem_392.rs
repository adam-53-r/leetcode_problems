use std::ops::Not;

struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars();
        let mut s_char = s_chars.next();

        for c in t.chars() {
            if let Some(x) = s_char {
                if c == x {
                    s_char = s_chars.next();
                }
            }
            else {
                return true
            }
        }
        s_char.is_none()
    }
}


fn main() {
    println!("{}", Solution::is_subsequence("abc".into(), "ahbgdc".into()));
}
