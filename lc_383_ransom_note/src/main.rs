/*
https://leetcode.com/problems/ransom-note/description/
Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

Each letter in magazine can only be used once in ransomNote.
*/
struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_chars: Vec<char> = magazine.chars().collect();
        let mut ransom_note_chars: Vec<char> = ransom_note.chars().collect();
        while let Some(letter) = ransom_note_chars.pop() {
            if let Some(pos) = magazine_chars.iter().position(|&x| x == letter) {
                magazine_chars.remove(pos);
            } else {
                return false;
            }
        }
        return true;
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
        assert_eq!(
            Solution::can_construct("a".to_string(), "b".to_string()),
            false
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::can_construct("aa".to_string(), "ab".to_string()),
            false
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::can_construct("aa".to_string(), "aab".to_string()),
            true
        );
    }

    #[test]
    fn empty_ransom_note() {
        assert_eq!(
            Solution::can_construct("".to_string(), "abc".to_string()),
            true
        );
    }

    #[test]
    fn empty_magazine() {
        assert_eq!(
            Solution::can_construct("a".to_string(), "".to_string()),
            false
        );
    }

    #[test]
    fn both_empty() {
        assert_eq!(
            Solution::can_construct("".to_string(), "".to_string()),
            true
        );
    }

    #[test]
    fn identical_strings() {
        assert_eq!(
            Solution::can_construct("abc".to_string(), "abc".to_string()),
            true
        );
    }

    #[test]
    fn magazine_shorter_than_ransom_note() {
        assert_eq!(
            Solution::can_construct("aab".to_string(), "ab".to_string()),
            false
        );
    }

    #[test]
    fn duplicate_letters_exact_match() {
        assert_eq!(
            Solution::can_construct("aabbcc".to_string(), "abcabc".to_string()),
            true
        );
    }

    #[test]
    fn magazine_missing_one_needed_letter() {
        assert_eq!(
            Solution::can_construct("aabbcc".to_string(), "abbcc".to_string()),
            false
        );
    }
}
