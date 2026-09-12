//Definition for singly-linked list.
//https://leetcode.com/problems/reverse-linked-list/description/
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
struct Solution;
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None;
        let mut current = head;
        // 1 2 3 Для второго прхода
        while let Some(mut node) = current {
            //забираем следующий элемент и пока забываем про него
            let next = node.next.take(); // 3
            node.next = prev; // prev - 1
            prev = Some(node); // prev - 2
            current = next; // next - 3
        }

        prev
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    pub struct ListBuilder {
        head: Option<Box<ListNode>>,
    }

    impl ListBuilder {
        pub fn new(val: i32) -> Self {
            ListBuilder {
                head: Some(Box::new(ListNode::new(val))),
            }
        }

        pub fn next(mut self, val: i32) -> Self {
            let mut current = self.head.as_mut().unwrap();

            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }

            current.next = Some(Box::new(ListNode::new(val)));
            self
        }

        pub fn build(self) -> Option<Box<ListNode>> {
            self.head
        }
    }

    #[test]
    fn test_multiple_elements() {
        let head = ListBuilder::new(1).next(2).next(3).next(4).next(5).build();
        let expected = ListBuilder::new(5).next(4).next(3).next(2).next(1).build();

        let actual = Solution::reverse_list(head);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two_elements() {
        let head = ListBuilder::new(1).next(2).build();
        let expected = ListBuilder::new(2).next(1).build();

        let actual = Solution::reverse_list(head);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_single_element() {
        let head = ListBuilder::new(1).build();
        let expected = ListBuilder::new(1).build();

        let actual = Solution::reverse_list(head);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_empty_list() {
        let head: Option<Box<ListNode>> = None;

        let actual = Solution::reverse_list(head);
        assert_eq!(None, actual);
    }
}
