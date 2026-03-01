use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

struct Solution1;

impl Solution1 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests1 {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution1::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}



struct Solution2;

impl Solution2 {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;

            if let Some(&idx) = map.get(&complement) {
                return vec![idx as i32, i as i32];
            }

            map.insert(num, i);
        }

        vec![]
    }
}


#[cfg(test)]
mod tests2 {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = Solution2::two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}