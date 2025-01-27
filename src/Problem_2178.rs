struct Solution {}

impl Solution {
    pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
        if final_sum % 2 != 0 { return vec![] }

        let mut ans = Vec::new();
        let mut cur_sum = 0;
        for n in (2..=final_sum).step_by(2) {
            if cur_sum + n > final_sum {
                *ans.last_mut().unwrap() += final_sum - cur_sum;
                break;
            } else {
                ans.push(n);
            }
            cur_sum += n;
        }

        ans
    }
}


fn main() {
    println!("{:?}", Solution::maximum_even_split(8));
}
