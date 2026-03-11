use std::rc::Rc;
use std::cell::RefCell;
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
struct Solution;
impl Solution {
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root.clone() {
            let mut n = node.borrow_mut();

            let left = n.left.take();
            let right = n.right.take();

            n.left = Self::invert_tree(right);
            n.right = Self::invert_tree(left);
        }

        root
    }
}


fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    struct TreeBuilder;

    impl TreeBuilder {
        // Build tree from level-order array like LeetCode
        fn from_vec(data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
            if data.is_empty() || data[0].is_none() {
                return None;
            }

            let root = Rc::new(RefCell::new(TreeNode::new(data[0].unwrap())));
            let mut queue = VecDeque::new();
            queue.push_back(root.clone());

            let mut i = 1;

            while i < data.len() {
                let node = queue.pop_front().unwrap();

                if let Some(Some(v)) = data.get(i) {
                    let left = Rc::new(RefCell::new(TreeNode::new(*v)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;

                if i < data.len() {
                    if let Some(Some(v)) = data.get(i) {
                        let right = Rc::new(RefCell::new(TreeNode::new(*v)));
                        node.borrow_mut().right = Some(right.clone());
                        queue.push_back(right);
                    }
                    i += 1;
                }
            }

            Some(root)
        }

        // Convert tree back to level-order Vec
        fn to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<i32>> {
            let mut result = Vec::new();
            let mut queue = VecDeque::new();

            queue.push_back(root);

            while let Some(node_opt) = queue.pop_front() {
                if let Some(node) = node_opt {
                    let n = node.borrow();
                    result.push(Some(n.val));
                    queue.push_back(n.left.clone());
                    queue.push_back(n.right.clone());
                } else {
                    result.push(None);
                }
            }

            // trim trailing None values
            while result.last() == Some(&None) {
                result.pop();
            }

            result
        }
    }

    #[test]
    fn test_case_1() {
        let root = TreeBuilder::from_vec(vec![
            Some(4),
            Some(2), Some(7),
            Some(1), Some(3), Some(6), Some(9)
        ]);

        let inverted = Solution::invert_tree(root);

        let result = TreeBuilder::to_vec(inverted);

        assert_eq!(
            result,
            vec![
                Some(4),
                Some(7), Some(2),
                Some(9), Some(6), Some(3), Some(1)
            ]
        );
    }
}
