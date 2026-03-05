fn main() {
    println!("Hello, world!");
}
struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut cleaned: Vec<char> = Vec::with_capacity(s.len());

        for c in s.chars() {
            if c.is_alphanumeric() {
                cleaned.extend(c.to_lowercase()); // handles multi-char lowercase
            }
        }

        if cleaned.len() < 2 {
            return true;
        }

        let mut left = 0;
        let mut right = cleaned.len() - 1;

        while left < right {
            if cleaned[left] != cleaned[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let s = "A man, a plan, a canal: Panama";
        assert_eq!(true, Solution::is_palindrome(s.to_string()));
    }
}
