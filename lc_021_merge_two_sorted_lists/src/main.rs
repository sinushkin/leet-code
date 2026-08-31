use std::collections::VecDeque;

// Definition for singly-linked list.
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
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut head1 = &mut list1;
        let mut head2 = &mut list2;
        let mut result : VecDeque<Box<ListNode>> = VecDeque::new();

        while head1.is_some() && head2.is_some() {
            if head1.as_mut().unwrap().val <= head2.as_mut().unwrap().val {
                result.push_back(head1.as_ref().unwrap().clone());
                head1 = &mut head1.as_mut().unwrap().next;
            }
            else if head2.as_mut().unwrap().val <= head1.as_mut().unwrap().val {
                result.push_back(head2.as_ref().unwrap().clone());
                head2 = &mut head2.as_mut().unwrap().next;
            }
        }
        //drain the rest
        if head1.is_some() {
            result.push_back(head1.as_ref().unwrap().clone());
            head1 = &mut head1.as_mut().unwrap().next;
        }
        if head2.is_some() {
            result.push_back(head2.as_ref().unwrap().clone());
            head2 = &mut head2.as_mut().unwrap().next;
        }

        //setup next pointer
        if !result.is_empty() {
          let mut head = result.pop_front().unwrap();
            let mut current = &mut head;
            while let Some(node) = result.pop_front() {
                current.next = Some(node);
                current = current.next.as_mut().unwrap();
            }
            Some(head)
        } else{
            None
        }
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
    fn test_case_1() {
        let list1 = ListBuilder::new(1)
            .next(2)
            .next(4)
            .build();

        let list2 = ListBuilder::new(1)
            .next(3)
            .next(4)
            .build();

        let expected = ListBuilder::new(1)
            .next(1)
            .next(2)
            .next(3)
            .next(4)
            .next(4)
            .build();

        let actual = Solution::merge_two_lists(list1, list2);
        assert_eq!(expected, actual);

    }
}
