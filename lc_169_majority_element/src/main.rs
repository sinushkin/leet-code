/*
https://leetcode.com/problems/majority-element/description/
Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.



Example 1:

Input: nums = [3,2,3]
Output: 3
Example 2:

Input: nums = [2,2,1,1,1,2,2]
Output: 2

*/

use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut map: HashMap<i32, usize> = HashMap::new();
        for n in nums.into_iter() {
            *map.entry(n).or_default() += 1;
        }
        let mut pairs: Vec<(i32, usize)> = map.into_iter().collect();
        pairs.sort_by(|a, b| b.1.cmp(&a.1));
        pairs[0].0
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
        assert_eq!(Solution::majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn single_element() {
        assert_eq!(Solution::majority_element(vec![1]), 1);
    }

    #[test]
    fn all_same_element() {
        assert_eq!(Solution::majority_element(vec![5, 5, 5, 5]), 5);
    }

    #[test]
    fn negative_numbers() {
        assert_eq!(Solution::majority_element(vec![-1, -1, -1, 2, 2]), -1);
    }

    #[test]
    fn majority_at_end() {
        assert_eq!(Solution::majority_element(vec![1, 2, 3, 3, 3]), 3);
    }

    #[test]
    fn two_elements_majority_first() {
        assert_eq!(Solution::majority_element(vec![1, 1, 2]), 1);
    }
}
