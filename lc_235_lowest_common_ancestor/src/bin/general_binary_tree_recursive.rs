/*
Второй правильный вариант — общий, не завязанный на то, что дерево
является BST (это решение LC 236, оно же годится и для 235, если бы
свойство "слева меньше, справа больше" не было гарантировано).

Ключевое отличие от main.rs: сравнение узлов идёт не по значению
(val), а по структурной идентичности через Rc::ptr_eq. Это важно —
в произвольном дереве значения могут повторяться, и совпадение чисел
само по себе ничего не говорит о том, один ли это узел. LCA — это
вопрос про структуру графа, а не про то, какие числа в нём лежат,
поэтому "текстовое" сравнение по val принципиально не подходит.

Идея рекурсии (снизу вверх):
- если текущий узел — это p или q, вернуть его;
- рекурсивно спуститься в left и в right;
- если из обоих поддеревьев что-то нашлось — p и q разошлись именно
  здесь, текущий узел и есть LCA;
- если нашлось только с одной стороны — просто передать находку выше,
  не трогая её.
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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (p, q) = (p?, q?);
        Self::helper(root, &p, &q)
    }

    fn helper(
        node: Option<Rc<RefCell<TreeNode>>>,
        p: &Rc<RefCell<TreeNode>>,
        q: &Rc<RefCell<TreeNode>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let current = node?;

        if Rc::ptr_eq(&current, p) || Rc::ptr_eq(&current, q) {
            return Some(current);
        }

        let left = Self::helper(current.borrow().left.clone(), p, q);
        let right = Self::helper(current.borrow().right.clone(), p, q);

        match (left, right) {
            (Some(_), Some(_)) => Some(current),
            (Some(found), None) => Some(found),
            (None, Some(found)) => Some(found),
            (None, None) => None,
        }
    }
}

fn main() {
    println!(
        "см. тесты: cargo test --bin general_binary_tree_recursive (если подключить как [[bin]])"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // Только для тестов: находит узел по значению, чтобы получить
    // тот самый Rc, на который потом сравниваем через ptr_eq.
    // Сам алгоритм LCA этой функцией не пользуется.
    fn find_node_by_value(
        node: &Option<Rc<RefCell<TreeNode>>>,
        value: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let current = node.as_ref()?;
        if current.borrow().val == value {
            return Some(current.clone());
        }
        find_node_by_value(&current.borrow().left, value)
            .or_else(|| find_node_by_value(&current.borrow().right, value))
    }

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
    fn test_case_1_lca_2_and_8() {
        let root = build_tree(&[
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = Solution::find_node_by_value(&root, 2);
        let q = Solution::find_node_by_value(&root, 8);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 6);
    }

    #[test]
    fn test_case_2_lca_2_and_4() {
        let root = build_tree(&[
            Some(6),
            Some(2),
            Some(8),
            Some(0),
            Some(4),
            Some(7),
            Some(9),
            None,
            None,
            Some(3),
            Some(5),
        ]);
        let p = Solution::find_node_by_value(&root, 2);
        let q = Solution::find_node_by_value(&root, 4);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_case_3_lca_2_and_1() {
        let root = build_tree(&[Some(2), Some(1)]);
        let p = Solution::find_node_by_value(&root, 2);
        let q = Solution::find_node_by_value(&root, 1);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
