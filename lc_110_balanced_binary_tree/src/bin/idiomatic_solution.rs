/*
Идиоматичное для Раста решение LC 110, как его обычно пишут на
leetcode discuss / в подборках "Rust solutions".

Отличия от main.rs:

1. Вспомогательная функция принимает `&Option<Rc<RefCell<TreeNode>>>`
   напрямую — без `.unwrap()` / `.is_none()` / `.as_ref().unwrap()`.
   Всё раскрытие делается одним `match`, включая случай `None`
   (пустое поддерево, высота 0).

2. Вместо `Option<usize>` для "признака дисбаланса" используется
   классический сигнальный трюк — `-1` как значение "дальше можно не
   считать, дерево уже не сбалансировано". Это позволяет одной
   функцией одновременно и мерить высоту, и распространять сигнал
   ошибки вверх, без отдельного типа Option/Result и без `?`.
   (Идея та же, что в вашем `Option<usize>` + `?`, просто в самой
   расхожей для литкодеров форме.)

3. Один проход по дереву снизу вверх — O(n) по времени, O(h) по
   стеку, никаких промежуточных коллекций.
*/

use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Self::height(&root) != -1
    }

    fn height(node: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let Some(n) = node else {
            return 0;
        };

        let n = n.borrow();
        let left = Self::height(&n.left);
        if left == -1 {
            return -1;
        }
        let right = Self::height(&n.right);
        if right == -1 || (left - right).abs() > 1 {
            return -1;
        }

        1 + left.max(right)
    }
}

fn main() {
    println!("см. тесты: cargo test --bin idiomatic_solution");
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

    #[test]
    fn test_case_1_balanced_tree() {
        let root = build_tree(&[Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_case_2_unbalanced_tree() {
        let root = build_tree(&[
            Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4),
        ]);
        assert_eq!(Solution::is_balanced(root), false);
    }

    #[test]
    fn test_case_3_empty_tree() {
        let root = build_tree(&[]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_case_4_single_node() {
        let root = build_tree(&[Some(1)]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    #[test]
    fn test_case_5_asymmetric_but_balanced() {
        let root = build_tree(&[Some(1), Some(2), Some(3), Some(4)]);
        assert_eq!(Solution::is_balanced(root), true);
    }

    /*
              1
             / \
           102 202
           /      \
         101      201
         /          \
       100          200

    Разница высот на уровне корня равна 0, но каждая ветка сама по
    себе — несбалансированная цепочка. Проверяет, что дисбаланс
    ловится на любой глубине, а не только у корня.
    */
    #[test]
    fn test_case_6_deep_imbalance_hidden_at_root_level() {
        let root = build_tree(&[
            Some(1), Some(102), Some(202), Some(101), None, None, Some(201), Some(100), None,
            None, Some(200),
        ]);
        assert_eq!(Solution::is_balanced(root), false);
    }
}
