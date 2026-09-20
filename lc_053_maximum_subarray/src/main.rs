// https://leetcode.com/problems/maximum-subarray/description/
// https://www.youtube.com/watch?v=lq8KOs1Ujas
// Идея
//
// Если накопленная сумма c текущим элементом меньше текущего элемента,
// то она только мешает текущему элементу.
// Выгоднее начать новый подмассив с текущего элемента.
//

struct Solution;
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            panic!("wrong nums size")
        }
        let mut global_max_sum = nums[0];
        let mut current_max_sum = nums[0];
        for n in nums.into_iter().skip(1) {
            current_max_sum = n.max(current_max_sum + n);
            global_max_sum = global_max_sum.max(current_max_sum);
        }
        global_max_sum
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
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
    }
}
