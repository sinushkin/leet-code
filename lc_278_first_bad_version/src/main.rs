// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)
struct Solution {
    bad: i32,
}

impl Solution {
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad
    }
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 0;
        let mut right = n;
        let mut mid = n / 2;
        loop {
            if self.isBadVersion(mid) {
                right = mid;
            } else {
                left = mid;
            }

            if right - left == 1 {
                return right;
            }
            mid = left + (right - mid) / 2;
        }
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
        let solution = Solution { bad: 4 };
        assert_eq!(solution.first_bad_version(5), 4);
    }

    #[test]
    fn example_2() {
        let solution = Solution { bad: 1 };
        assert_eq!(solution.first_bad_version(1), 1);
    }

    #[test]
    fn last_executed_input() {
        let solution = Solution { bad: 1 };
        assert_eq!(solution.first_bad_version(4), 1);
    }
}
