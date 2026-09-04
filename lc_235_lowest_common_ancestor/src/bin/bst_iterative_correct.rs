/*





Как это решают лит-кодеры для LC 235 (LCA in a BST) — идиоматичный
и самый частый принятый вариант.

Идея в том, чтобы вообще не строить никаких вспомогательных структур
("текстовых" представлений пути вроде набора чисел). Вместо этого
используется само свойство binary search tree прямо во время спуска:

- если и p, и q меньше значения текущего узла — оба они лежат в левом
  поддереве, спускаемся налево;
- если и p, и q больше — оба лежат в правом поддереве, спускаемся
  направо;
- как только это перестаёт выполняться (p и q оказались по разные
  стороны, либо один из них равен текущему узлу) — текущий узел и
  есть LCA: это первая точка, где пути к p и q расходятся.

Никакого пересечения множеств, никакого .max() — ответ находится
за один проход вниз по дереву, за O(h) без дополнительной памяти.
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
// Просто от корня ищем либо одну либо другую ноду - она и будет LCA
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p?.borrow().val;
        let q_val = q?.borrow().val;
        let mut node = root;

        while let Some(current) = node {
            let current_val = current.borrow().val;
            node = if p_val < current_val && q_val < current_val {
                current.borrow().left.clone()
            } else if p_val > current_val && q_val > current_val {
                current.borrow().right.clone()
            } else {
                return Some(current);
            };
        }
        None
    }
}

fn main() {
    println!("см. тесты: cargo test --bin bst_iterative_correct (если подключить как [[bin]])");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    fn find_node(root: &Rc<RefCell<TreeNode>>, value: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = Some(root.clone());
        while let Some(n) = node {
            let current_val = n.borrow().val;
            node = if value < current_val {
                n.borrow().left.clone()
            } else if value > current_val {
                n.borrow().right.clone()
            } else {
                return Some(n);
            };
        }
        None
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
        ])
        .unwrap();
        let p = find_node(&root, 2);
        let q = find_node(&root, 8);
        let result = Solution::lowest_common_ancestor(Some(root), p, q);
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
        ])
        .unwrap();
        let p = find_node(&root, 2);
        let q = find_node(&root, 4);
        let result = Solution::lowest_common_ancestor(Some(root), p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_case_3_lca_2_and_1() {
        let root = build_tree(&[Some(2), Some(1)]).unwrap();
        let p = find_node(&root, 2);
        let q = find_node(&root, 1);
        let result = Solution::lowest_common_ancestor(Some(root), p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
