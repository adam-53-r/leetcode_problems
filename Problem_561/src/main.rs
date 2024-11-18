struct Solution {}

impl Solution {
    pub fn array_pair_sum(nums: Vec<i32>) -> i32 {
        
        let mut out = 0;
        
        let mut second = nums.iter().peekable();
        second.next();
        for x in 0..nums.len()-1 {
            let mut temp = 0;
            print!("({}, {}), ", nums[0], second.peek().unwrap());
            temp = temp + std::cmp::min(nums[0], *second.next().unwrap());
            let mut other = second.clone();
            let mut other2 = second.clone();
            for y in 0..(nums.len()/2)-1 {
                
                temp = temp + std::cmp::min(
                    other.next().unwrap_or_else(|| {
                        other = nums.iter().peekable();
                        other.next();
                        return other.next().unwrap();
                    }),
                    other.next().unwrap_or_else(|| {
                        other = nums.iter().peekable();
                        print!("{}", other.next().unwrap());
                        return other.next().unwrap();
                    }),
                );
                print!("({}, {}), ",
                    other2.next().unwrap_or_else(|| {
                        other = nums.iter().peekable();
                        other.next();
                        return other.next().unwrap();
                    }),
                    other2.next().unwrap_or_else(|| {
                        other = nums.iter().peekable();
                        other.next();
                        return other.next().unwrap();
                    }),
                );
            }
            println!();
            out = std::cmp::max(out, temp);
            
        }
        return out;
    }
}

fn main() {
    println!("{}", Solution::array_pair_sum(vec!(6,9,6,3,8,4)) );
}
