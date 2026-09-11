// https://leetcode.com/problems/longest-palindrome/
//
/*
Example 1:

Input: s = "abccccdd"
Output: 7
Explanation: One longest palindrome that can be built is "dccaccd", whose length is 7.
Example 2:

Input: s = "a"
Output: 1
Explanation: The longest palindrome that can be built is "a", whose length is 1.
*/
use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        s.chars().for_each(|c| {
            *map.entry(c).or_insert(0) += 1;
        });
        let mut pairs: Vec<(&char, &usize)> = map.iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(a.1));
        let mut single_chair_exists = false;
        let mut result = 0;
        for pair in pairs {
            let to_add = if pair.1 % 2 == 0 {
                *pair.1
            } else if *pair.1 > 2 {
                single_chair_exists = true;
                pair.1 - 1
            } else {
                single_chair_exists = true;
                0
            };
            result += to_add;
        }
        (result + if single_chair_exists { 1 } else { 0 }) as i32
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::longest_palindrome("abccccdd".to_string()), 7);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn odd_count_keeps_even_part() {
        assert_eq!(Solution::longest_palindrome("aaaaa".to_string()), 5);
    }
}
