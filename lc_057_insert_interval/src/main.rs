//https://leetcode.com/problems/insert-interval/
//
struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut start = new_interval[0];
        let mut end = new_interval[1];
        let mut merged = false;
        for cur in intervals.into_iter() {
            let cursor_start = cur[0];
            let cursor_end = cur[1];
            // Интервал целиком левее чем целевой интервал
            if cursor_end < start {
                result.push(cur);
            } else if cursor_start > end {
                // целиком правее
                if !merged {
                    result.push(vec![start, end]);
                    merged = true;
                }
                result.push(cur);
            } else {
                start = cursor_start.min(start);
                end = cursor_end.max(end);
            }
        }
        if !merged {
            result.push(vec![start, end]);
        }
        result
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
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            vec![vec![1, 2], vec![3, 10], vec![12, 16]]
        );
    }

    #[test]
    fn empty_intervals() {
        assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![vec![5, 7]]);
    }

    #[test]
    fn insert_before_all() {
        assert_eq!(
            Solution::insert(vec![vec![3, 5], vec![6, 9]], vec![1, 2]),
            vec![vec![1, 2], vec![3, 5], vec![6, 9]]
        );
    }

    #[test]
    fn insert_after_all() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![3, 5]], vec![6, 7]),
            vec![vec![1, 2], vec![3, 5], vec![6, 7]]
        );
    }

    #[test]
    fn insert_in_gap_without_overlap() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![6, 9]], vec![3, 4]),
            vec![vec![1, 2], vec![3, 4], vec![6, 9]]
        );
    }

    #[test]
    fn touches_left_boundary() {
        assert_eq!(
            Solution::insert(vec![vec![3, 5]], vec![1, 3]),
            vec![vec![1, 5]]
        );
    }

    #[test]
    fn touches_right_boundary() {
        assert_eq!(
            Solution::insert(vec![vec![3, 5]], vec![5, 8]),
            vec![vec![3, 8]]
        );
    }

    #[test]
    fn inside_existing_interval() {
        assert_eq!(
            Solution::insert(vec![vec![1, 10]], vec![3, 5]),
            vec![vec![1, 10]]
        );
    }

    #[test]
    fn covers_existing_interval() {
        assert_eq!(
            Solution::insert(vec![vec![3, 5]], vec![1, 8]),
            vec![vec![1, 8]]
        );
    }

    #[test]
    fn covers_all_intervals() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![3, 4], vec![5, 6]], vec![0, 10]),
            vec![vec![0, 10]]
        );
    }

    #[test]
    fn starts_in_gap_ends_inside_interval() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![4, 6], vec![8, 10]], vec![3, 5]),
            vec![vec![1, 2], vec![3, 6], vec![8, 10]]
        );
    }

    #[test]
    fn starts_inside_interval_ends_in_gap() {
        assert_eq!(
            Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            vec![vec![1, 5], vec![6, 9]]
        );
    }

    #[test]
    fn same_interval() {
        assert_eq!(
            Solution::insert(vec![vec![2, 4]], vec![2, 4]),
            vec![vec![2, 4]]
        );
    }

    #[test]
    fn point_interval() {
        assert_eq!(
            Solution::insert(vec![vec![1, 2], vec![4, 5]], vec![3, 3]),
            vec![vec![1, 2], vec![3, 3], vec![4, 5]]
        );
    }
}
