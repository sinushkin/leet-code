use std::collections::VecDeque;

fn main() {
    println!("Hello, world!");
}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for c in s.chars(){
            match c {
                '{' | '[' | '(' => stack.push(c),
                '}' => {
                    if Some('{') != stack.pop() {
                        return false;
                    }
                }
                ']' => {
                    if Some('[') != stack.pop() {
                        return false;
                    }
                }
                ')' => {
                    if Some('(') != stack.pop() {
                        return false;
                    }
                }
                _=> panic!("Invalid char: {}", c),
            }
        }
        stack.is_empty()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let result = Solution::is_valid("(){}[]".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_2() {
        let result = Solution::is_valid("{()[]}".to_string());
        assert_eq!(result, true);
    }
    #[test]
    fn test_case_3() {
        let result = Solution::is_valid("{()[}]".to_string());
        assert_eq!(result, false);
    }
}
