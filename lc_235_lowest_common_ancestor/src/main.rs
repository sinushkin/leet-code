// Definition for a binary tree node.
// https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/description/
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
use std::collections::HashSet;
use std::fmt::LowerExp;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return None;
        }
        if p.is_none() && q.is_some() {
            return q.clone();
        }
        if q.is_none() && p.is_some() {
            return p.clone();
        }
        let (lowest, highest) = {
            let p = p.as_ref().unwrap().clone();
            let q = q.as_ref().unwrap().clone();

            if p.borrow().val < q.borrow().val {
                (p, q)
            } else if q.borrow().val < p.borrow().val {
                (q, p)
            } else {
                return Some(p);
            }
        };
        let root = root.as_ref().unwrap().clone();
        let root_value = root.borrow().val.clone();
        let lowest_ancestors = Self::fill_chain(root.clone(), lowest.borrow().val);
        let highest_ancestors = Self::fill_chain(root.clone(), highest.borrow().val);

        let intersection = highest_ancestors.intersection(&lowest_ancestors);
        let max_value = intersection.max().copied();

        match max_value {
            Some(value) => Self::find_node(root, value),
            None => None,
        }
    }
    fn fill_chain(root: Rc<RefCell<TreeNode>>, value: i32) -> HashSet<i32> {
        let mut lowest_ancestors = HashSet::new();
        let mut node = Some(root.clone());
        lowest_ancestors.insert(root.borrow().val);
        while let Some(n) = node {
            if value < n.borrow().val {
                let left_node = n.borrow().left.clone().unwrap();
                lowest_ancestors.insert(left_node.borrow().val);
                node = Some(left_node);
            } else if value > n.borrow().val {
                let right_node = n.borrow().right.clone().unwrap();
                lowest_ancestors.insert(right_node.borrow().val);
                node = Some(right_node);
            } else {
                lowest_ancestors.insert(n.borrow().val);
                break;
            }
        }
        lowest_ancestors
    }
    fn find_node(root: Rc<RefCell<TreeNode>>, value: i32) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = Some(root);
        while let Some(n) = node {
            if value < n.borrow().val {
                node = n.borrow().left.clone();
            } else if value > n.borrow().val {
                node = n.borrow().right.clone();
            } else {
                return Some(n.clone());
            }
        }
        None
    }
}
fn main() {
    println!("Hello, world!");
}

// Почему так нельзя делать: test_case_2 ниже падает не из-за опечатки,
// а из-за того, как устроен сам алгоритм.
//
// `fill_chain` превращает путь от корня до узла в HashSet<i32> —
// то есть в "дерево как текст": остаётся только набор чисел-значений,
// а порядок (кто чей родитель, кто ближе к корню) теряется полностью.
// HashSet вообще не хранит порядок вставки.
//
// LCA — понятие структурное: это самый глубокий узел, через который
// проходят оба пути от корня. У пересечения двух путей эта глубина
// определяется положением в дереве, а не значением val. Взять
// `.max()` от пересечения работает только случайно — если в вашем
// дереве более глубокие общие предки всегда имеют бОльшие значения.
// Для p=2, q=4 общие предки — {6, 2}, и .max() берёт 6 (корень),
// хотя правильный ответ — 2, самый глубокий общий предок.
//
// Правильные решения (см. отдельные файлы в src/) либо используют
// упорядоченность BST прямо во время спуска по дереву (не строя
// никаких множеств), либо рекурсивно сравнивают структурную
// идентичность узлов (Rc::ptr_eq), а не текстовое/числовое значение.
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
        let p = Solution::find_node(root.clone().unwrap(), 2);
        let q = Solution::find_node(root.clone().unwrap(), 8);
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
        let p = Solution::find_node(root.clone().unwrap(), 2);
        let q = Solution::find_node(root.clone().unwrap(), 4);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }

    #[test]
    fn test_case_3_lca_2_and_1() {
        let root = build_tree(&[Some(2), Some(1)]);
        let p = Solution::find_node(root.clone().unwrap(), 2);
        let q = Solution::find_node(root.clone().unwrap(), 1);
        let result = Solution::lowest_common_ancestor(root, p, q);
        assert_eq!(result.unwrap().borrow().val, 2);
    }
}
