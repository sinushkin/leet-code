#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut size = 0;
        let mut node = &head;
        while let Some(n) = node {
            size += 1;
            node = &n.next;
        }
        let middle = size / 2 + 1;
        node = &head;
        let mut counter = 0;
        while let Some(n) = node {
            counter += 1;
            if counter == middle {
                return Some(n.clone());
            }
            node = &n.next;
        }
        None
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn from_vec(values: &[i32]) -> Option<Box<ListNode>> {
        let mut head = None;
        for &val in values.iter().rev() {
            head = Some(Box::new(ListNode { val, next: head }));
        }
        head
    }

    fn to_vec(mut node: Option<Box<ListNode>>) -> Vec<i32> {
        let mut values = Vec::new();
        while let Some(n) = node {
            values.push(n.val);
            node = n.next;
        }
        values
    }

    #[test]
    fn example_1() {
        let head = from_vec(&[1, 2, 3, 4, 5]);
        let result = Solution::middle_node(head);
        assert_eq!(to_vec(result), vec![3, 4, 5]);
    }

    #[test]
    fn example_2() {
        let head = from_vec(&[1, 2, 3, 4, 5, 6]);
        let result = Solution::middle_node(head);
        assert_eq!(to_vec(result), vec![4, 5, 6]);
    }
}
