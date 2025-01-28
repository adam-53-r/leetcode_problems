struct Solution {}

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        word1
            .chars()
            // Joining the two iterators of characters so we get the elements of both in pairs.
            .zip(word2.chars())
            // Putting the characters in slices and joining all the slices in a single one
            // so we get the strings merged how we want but only up to the last pair of
            // characters. The end of the longer string (if there is any) is still missing.
            .flat_map(|(c1, c2)| [c1, c2])
            // When chaining the characters of both words like this, only one
            // of two things can happen:
            // 1. Both words are equal in length so they cancel each other out when skipping
            //    n items where n is the length of the other word.
            // 2. One work i longer than the other, so the smaller word gets skipped
            //    completely, and the longer word gets skipped only up to the same length
            //    as the shorter one, thus giving us an iterator of the longer word with the
            //    missing characters that didn't have a pair with the zip iterator.
            .chain(word1.chars().skip(word2.len()))
            .chain(word2.chars().skip(word1.len()))
            .collect()
    }
}


fn main() {
    println!("{}", Solution::merge_alternately("ab".into(), "pqrs".into()));
}








// let mut merge = word1.chars()
//     .zip(word2.chars())
//     .map(|c| {
//         c.0.to_string() + &c.1.to_string()
//     })
//     .reduce(|s, e| {
//         s + &e
//     }).unwrap();
//
// if word1 > word2 {
//     merge.push_str(
//         word1.get( word1.len() - word2.len().. ).unwrap()
//     )
// }
// else {
//     merge.push_str(
//         word2.get( word2.len() - word1.len().. ).unwrap()
//     )
// }
// merge