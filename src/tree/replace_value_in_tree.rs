use std::cell::RefCell;
use std::rc::Rc;

use crate::tree::{Solution, TreeNode};

// https://leetcode.cn/problems/cousins-in-binary-tree-ii/description/
impl Solution {
    pub fn replace_value_in_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            node.borrow_mut().val = 0;
            let mut q = vec![node.clone()];

            while !q.is_empty() {
                let mut nxt = Vec::new();
                let mut next_level_sum = 0;

                for node in &q {
                    let mut borrowed_node = node.borrow_mut();

                    // 第一次遍历
                    if let Some(left) = borrowed_node.left.clone() {
                        nxt.push(left.clone());
                        next_level_sum += left.borrow().val;
                    }

                    if let Some(right) = borrowed_node.right.clone() {
                        nxt.push(right.clone());
                        next_level_sum += right.borrow().val;
                    }
                }

                for node in &q {
                    let mut borrowed_node = node.borrow_mut();

                    // 第二次遍历，排除自己和自己的子
                    let children_sum = borrowed_node.left.as_ref().map(|n| n.borrow().val).unwrap_or(0)
                        + borrowed_node.right.as_ref().map(|n| n.borrow().val).unwrap_or(0);

                    if let Some(left) = borrowed_node.left.clone() {
                        left.borrow_mut().val = next_level_sum - children_sum;
                    }

                    if let Some(right) = borrowed_node.right.clone() {
                        right.borrow_mut().val = next_level_sum - children_sum;
                    }
                }

                q = nxt
            }
            Some(node)
        } else {
            None
        }
    }
}

#[test]
pub fn test_it() {}