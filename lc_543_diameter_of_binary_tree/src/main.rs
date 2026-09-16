//Definition for a binary tree node.
//https://leetcode.com/problems/diameter-of-binary-tree/
/*
Given the root of a binary tree, return the length of the diameter of the tree.
The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.
The length of a path between two nodes is represented by the number of edges between them.
*/
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

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_diameter = 0;

        if let Some(node) = &root {
            Self::get_deepness(node, &mut max_diameter);
        }

        max_diameter
    }

    fn get_deepness(node: &Rc<RefCell<TreeNode>>, max_diameter: &mut i32) -> i32 {
        let left_deepness = if let Some(left) = &node.borrow().left {
            Self::get_deepness(left, max_diameter)
        } else {
            0
        };
        let right_deepness = if let Some(right) = &node.borrow().right {
            Self::get_deepness(right, max_diameter)
        } else {
            0
        };
        *max_diameter = (*max_diameter).max(left_deepness + right_deepness);
        left_deepness.max(right_deepness) + 1
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node(val: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(val)))
    }

    #[test]
    fn example_1() {
        // root = [1,2,3,4,5] -> 3 ([4,2,1,3] or [5,2,1,3])
        let n4 = node(4);
        let n5 = node(5);
        let n2 = node(2);
        n2.borrow_mut().left = Some(n4);
        n2.borrow_mut().right = Some(n5);
        let n3 = node(3);
        let n1 = node(1);
        n1.borrow_mut().left = Some(n2);
        n1.borrow_mut().right = Some(n3);

        assert_eq!(Solution::diameter_of_binary_tree(Some(n1)), 3);
    }

    #[test]
    fn example_2() {
        // root = [1,2] -> 1
        let n2 = node(2);
        let n1 = node(1);
        n1.borrow_mut().left = Some(n2);

        assert_eq!(Solution::diameter_of_binary_tree(Some(n1)), 1);
    }

    #[test]
    fn empty_tree() {
        assert_eq!(Solution::diameter_of_binary_tree(None), 0);
    }

    #[test]
    fn single_node() {
        assert_eq!(Solution::diameter_of_binary_tree(Some(node(1))), 0);
    }
}
