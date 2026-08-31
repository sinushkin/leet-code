///https://leetcode.com/problems/binary-search/submissions/2126818523/
// Example 1:

// Input: nums = [-1,0,3,5,9,12], target = 9
// Output: 4
// Explanation: 9 exists in nums and its index is 4
struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0usize;
        let mut right: usize = nums.len();
        while left < right {
            let mid_point = ((right - left) / 2) + left;
            let mid_point_value = nums[mid_point];
            if mid_point_value == target {
                return mid_point as i32;
            }
            if target < mid_point_value {
                right = mid_point;
            } else if target > mid_point_value {
                left = mid_point + 1;
            }
        }
        -1
    }
}

fn main() {
    let index = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
    println!("{index}");

    let index = Solution::search(vec![-1, 0, 3, 5, 9, 12], 2);
    println!("{index}");

    let index = Solution::search(vec![5], 5);
    println!("{index}");
}

// pub fn search(nums: Vec<i32>, target: i32) -> i32 {
//     let mut left = 0usize;
//     let mut right = nums.len();

//     while left < right {
//         let mid = left + (right - left) / 2;

//         if nums[mid] == target {
//             return mid as i32;
//         }

//         if nums[mid] < target {
//             left = mid + 1;
//         } else {
//             right = mid;
//         }
//     }

//     -1
// }
