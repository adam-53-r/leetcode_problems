struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut other:Vec<i32> = vec![];
        let mut result:Vec<i32> = vec![];
        let mut map:std::collections::HashMap<i32,i32>= Default::default();

        for i in nums1{
            *map.entry(i).or_insert(0) += 1;            
        }
        other = nums2.clone();
        
        for i in other{
            if let Some(value) = map.get_mut(&i){
                result.push(i);
                *value -= 1;
                if *value == 0 {map.remove(&i);}
            }
        }
        return result;
    }
}


fn main() {
    println!("{:?}", Solution::intersect(vec![2,1], vec![1,2]));
}
