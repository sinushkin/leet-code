// https://leetcode.com/problems/contains-duplicate/description
use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut set = HashSet::new();
        for n in nums.iter() {
            // if set.contains(n) {
            //     return true;
            // }
            // set.insert(n);
            if !set.insert(n) {
                return true;
            }
        }
        false
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
        assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn example_2() {
        assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn example_3() {
        assert!(Solution::contains_duplicate(vec![
            1, 1, 1, 3, 3, 4, 3, 2, 4, 2
        ]));
    }

    #[test]
    fn single_element() {
        assert!(!Solution::contains_duplicate(vec![1]));
    }

    #[test]
    fn two_equal_elements() {
        assert!(Solution::contains_duplicate(vec![7, 7]));
    }

    #[test]
    fn two_different_elements() {
        assert!(!Solution::contains_duplicate(vec![7, 8]));
    }

    #[test]
    fn all_same_elements() {
        assert!(Solution::contains_duplicate(vec![5, 5, 5, 5]));
    }

    #[test]
    fn negative_numbers() {
        assert!(Solution::contains_duplicate(vec![-1, 2, -1]));
        assert!(!Solution::contains_duplicate(vec![-1, 1]));
    }

    #[test]
    fn extreme_values() {
        assert!(Solution::contains_duplicate(vec![i32::MIN, 0, i32::MIN]));
        assert!(!Solution::contains_duplicate(vec![i32::MIN, i32::MAX]));
    }

    #[test]
    fn duplicate_at_the_ends() {
        assert!(Solution::contains_duplicate(vec![9, 1, 2, 3, 9]));
    }

    #[test]
    fn duplicates_adjacent() {
        assert!(Solution::contains_duplicate(vec![1, 2, 2, 3]));
    }

    #[test]
    fn large_input_without_duplicates() {
        let nums: Vec<i32> = (0..100_000).collect();
        assert!(!Solution::contains_duplicate(nums));
    }

    #[test]
    fn large_input_with_duplicate_at_the_end() {
        let mut nums: Vec<i32> = (0..100_000).collect();
        nums.push(0);
        assert!(Solution::contains_duplicate(nums));
    }
}
