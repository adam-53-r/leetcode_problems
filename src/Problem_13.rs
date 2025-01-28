struct Solution {}

/*
I*             1
V             5
X*             10
L             50
C*             100
D             500
M             1000
 */

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .rfold(0, |acc, e| {
                acc + match e {
                    'I' if acc >= 5 => -1,
                    'I' => 1,
                    'V' => 5,
                    'X' if acc >= 50 => -10,
                    'X' => 10,
                    'L' => 50,
                    'C' if acc >= 500 => -100,
                    'C' => 100,
                    'D' => 500,
                    'M' => 1000,
                    _ => 0
                }
            })
    }
}


fn main() {
    println!("{}", Solution::roman_to_int("XL".into()));
    println!("{}", Solution::roman_to_int("L".into()));
    println!("{}", Solution::roman_to_int("LXII".into()));
    println!("{}", Solution::roman_to_int("XC".into()));
    println!("{}", Solution::roman_to_int("MCMXCIV".into()));
}
