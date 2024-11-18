struct Solution {}


impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        
        if s.len() == 1 {
            return false;
        }

        let S: Vec<char> = s.chars().collect();
        

        for x in 0..s.len() {
            for y in 0..s.len() {
                if x != y {
                    let mut temp = S.clone();
                    let temp_var = temp[x];
                    temp[x] = temp[y];
                    temp[y] = temp_var;
                    if temp.iter().collect::<String>() == goal {
                        return true;
                    }
                }
            }
        }

        false
    }
}


fn main() {
    println!("{}", Solution::buddy_strings(String::from("aa"), String::from("aa")));
}
