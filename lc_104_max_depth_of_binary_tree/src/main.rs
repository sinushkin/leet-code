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
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::rc::Rc;
struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            Self::get_depth(&node)
        } else {
            0
        }
    }
    fn get_depth(node: &Rc<RefCell<TreeNode>>) -> i32 {
        let left_depth = if let Some(node) = &node.borrow().left {
            Self::get_depth(node)
        } else {
            0
        };
        let right_depth = if let Some(node) = &node.borrow().right {
            Self::get_depth(node)
        } else {
            0
        };
        1 + left_depth.max(right_depth)
    }
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    fn node(
        val: i32,
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Some(Rc::new(RefCell::new(TreeNode { val, left, right })))
    }

    #[test]
    fn example_1() {
        // [3,9,20,null,null,15,7]
        let root = node(3, leaf(9), node(20, leaf(15), leaf(7)));
        assert_eq!(Solution::max_depth(root), 3);
    }

    #[test]
    fn example_2() {
        // [1,null,2]
        let root = node(1, None, leaf(2));
        assert_eq!(Solution::max_depth(root), 2);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::max_depth(None), 0);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::max_depth(leaf(42)), 1);
    }
}
