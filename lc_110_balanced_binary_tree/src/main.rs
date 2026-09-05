// Definition for a binary tree node.
// https://leetcode.com/problems/balanced-binary-tree/description/
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

//Сбалансированное дерева - это такое дерево в котором подветви по глубине не отличаются более чем на 1
/*

        3
       / \
      9   20
         /  \
        15   7

        1
       / \
      2   2
     / \
    3   3
   / \
  4   4
*/
struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }

        let node = root.unwrap();

        let left = if node.borrow().left.is_none() {
            Some(0)
        } else {
            Self::height(node.borrow().left.as_ref().unwrap()).map(|h| h + 1)
        };
        let right = if node.borrow().right.is_none() {
            Some(0)
        } else {
            Self::height(node.borrow().right.as_ref().unwrap()).map(|h| h + 1)
        };
        match (left, right) {
            (Some(l), Some(r)) => l.abs_diff(r) <= 1,
            _ => false,
        }
    }

    fn height(node: &Rc<RefCell<TreeNode>>) -> Option<usize> {
        let left = if node.borrow().left.is_none() {
            0
        } else {
            Self::height(node.borrow().left.as_ref().unwrap())? + 1
        };
        let right = if node.borrow().right.is_none() {
            0
        } else {
            Self::height(node.borrow().right.as_ref().unwrap())? + 1
        };
        if left.abs_diff(right) > 1 {
            None
        } else {
            Some(left.max(right))
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    fn build_tree(vals: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
        if vals.is_empty() || vals[0].is_none() {
            return None;
        }
        let root = Rc::new(RefCell::new(TreeNode::new(vals[0].unwrap())));
        let mut queue = VecDeque::new();
        queue.push_back(root.clone());
        let mut i = 1;
        while i < vals.len() {
            let node = queue.pop_front().unwrap();
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().left = Some(left.clone());
                    queue.push_back(left);
                }
                i += 1;
            }
            if i < vals.len() {
                if let Some(v) = vals[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(v)));
                    node.borrow_mut().right = Some(right.clone());
                    queue.push_back(right);
                }
                i += 1;
            }
        }
        Some(root)
    }

    /*
    Пример из условия — сбалансированное дерево

            3
           / \
          9   20
             /  \
            15   7

    Разница высот левого (9, высота 1) и правого (20, высота 2)
    поддеревьев корня равна 1 — в пределах допустимого.
    */
    #[test]
    fn test_case_1_balanced_tree() {
        let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    /*
    Пример из условия — несбалансированное дерево

            1
           / \
          2   2
         / \
        3   3
       / \
      4   4

    Левое поддерево (2) имеет высоту 3, правое (2, лист) — высоту 1.
    Разница 2 > 1 — дерево не сбалансировано.
    */
    #[test]
    fn test_case_2_unbalanced_tree() {
        let root = build_tree(&[
            Some(1),
            Some(2),
            Some(2),
            Some(3),
            Some(3),
            None,
            None,
            Some(4),
            Some(4),
        ]);
        assert_eq!(Solution::is_balanced(root), false);
    }

    /*
    Пустое дерево

        (nil)

    Пустое дерево считается сбалансированным по определению задачи.
    */
    #[test]
    fn test_case_3_empty_tree() {
        let root = build_tree(&[]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    /*
    Дерево из одного узла

        1

    Нет ни одного поддерева — разница высот 0.
    */
    #[test]
    fn test_case_4_single_node() {
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    /*
    Асимметричное, но сбалансированное дерево

            1
           / \
          2   3
         /
        4

    Левое поддерево (2) выше правого (3) на 1 — это ещё в пределах
    допустимого, дерево остаётся сбалансированным.
    */
    #[test]
    fn test_case_5_asymmetric_but_balanced() {
        let root = build_tree(&[Some(1), Some(2), Some(3), Some(4)]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    /*
    Правая цепочка, один ребёнок отсутствует на каждом уровне

        1
         \
          2
           \
            3

    LeetCode: root = [1,null,2,null,3] -> false.
    */
    #[test]
    fn test_case_7_right_skewed_chain() {
        let root = build_tree(&[Some(1), None, Some(2), None, Some(3)]);
        assert_eq!(Solution::is_balanced(root), false);
    }

    /*
    Дисбаланс спрятан глубоко, а не на уровне корня

              1
             / \
           102 202
           /      \
         101      201
         /          \
       100          200

    На уровне корня высоты левого (102) и правого (202) поддеревьев
    совпадают (обе цепочки длиной 3) — разница 0. Но каждое из этих
    поддеревьев само по себе является цепочкой и не сбалансировано
    (внутри 102: разница высот левого/правого потомка равна 2).
    Решение обязано провалиться сюда false, если оно действительно
    проверяет баланс рекурсивно на каждом узле, а не только сравнивает
    высоты двух поддеревьев корня.
    */
    #[test]
    fn test_case_6_deep_imbalance_hidden_at_root_level() {
        let root = build_tree(&[
            Some(1),
            Some(102),
            Some(202),
            Some(101),
            None,
            None,
            Some(201),
            Some(100),
            None,
            None,
            Some(200),
        ]);
        assert_eq!(Solution::is_balanced(root), false);
    }
}
