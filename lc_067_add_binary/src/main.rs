/*
https://leetcode.com/problems/add-binary/description/
Example 1:

Input: a = "11", b = "1"
Output: "100"
Example 2:

Input: a = "1010", b = "1011"
Output: "10101"

*/
struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let (long, short) = if a.len() > b.len() { (a, b) } else { (b, a) };
        let long_chars: Vec<char> = long.chars().collect();
        let short_chars: Vec<char> = short.chars().collect();
        let mut long_counter: i32 = long_chars.len() as i32 - 1;
        let mut next_long = || -> Option<bool> {
            if long_counter >= 0 {
                let result = long_chars[long_counter as usize] == '1';
                long_counter -= 1;
                Some(result)
            } else {
                None
            }
        };
        let mut short_counter: i32 = short_chars.len() as i32 - 1;
        let mut next_short = || -> bool {
            if short_counter >= 0 {
                let result = short_chars[short_counter as usize] == '1';
                short_counter -= 1;
                result
            } else {
                false
            }
        };

        let mut carry = false;
        let mut result = std::collections::VecDeque::new();
        while let Some(bit_1) = next_long() {
            let bit_2 = next_short();
            if !bit_1 && !bit_2 {
                result.push_front(if carry { "1" } else { "0" });
                carry = false;
            } else if bit_1 != bit_2 {
                result.push_front(if !carry { "1" } else { "0" });
            } else {
                result.push_front(if carry { "1" } else { "0" });
                carry = true;
            }
        }
        if carry {
            result.push_front("1");
        }
        result.into_iter().collect()
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(a: &str, b: &str, expected: &str) {
        assert_eq!(Solution::add_binary(a.to_string(), b.to_string()), expected);
    }

    #[test]
    fn example_1() {
        check("11", "1", "100");
    }

    #[test]
    fn example_2() {
        check("1010", "1011", "10101");
    }

    #[test]
    fn single_zeros() {
        check("0", "0", "0");
    }

    #[test]
    fn single_bits_no_carry() {
        check("1", "0", "1");
    }

    #[test]
    fn single_bits_with_carry() {
        check("1", "1", "10");
    }

    #[test]
    fn carry_propagates_through_all_ones() {
        check("111", "1", "1000");
    }

    #[test]
    fn equal_length_all_ones() {
        check("1111", "1111", "11110");
    }

    #[test]
    fn different_length_no_carry() {
        check("100", "1", "101");
    }
}
