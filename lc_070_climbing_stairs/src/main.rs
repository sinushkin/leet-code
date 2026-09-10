

// https://leetcode.com/problems/climbing-stairs/

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut stairs = [0i32; 46];
        stairs[1] = 1;
        stairs[2] = 2;
        stairs[3] = stairs[1] + stairs[2];
        for i in 4..=45 {
            stairs[i] = stairs[i - 1] + stairs[i - 2];
        }
        return stairs[n as usize];
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
        assert_eq!(Solution::climb_stairs(2), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::climb_stairs(3), 3);
    }
}
