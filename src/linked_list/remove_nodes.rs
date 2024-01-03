use crate::linked_list::{ListNode, Solution};

// https://leetcode.cn/problems/remove-nodes-from-linked-list/?envType=daily-question&envId=2024-01-03
impl Solution {
    pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            Some(mut boxed_head) => {
                if let Some(mut next_node) = boxed_head.next.take() {
                    let node = Self::remove_nodes(Some(next_node));
                    if boxed_head.val < node.as_ref().unwrap().val {
                        node
                    } else {
                        boxed_head.next = node;
                        Some(boxed_head)
                    }
                } else {
                    Some(boxed_head)
                }
            }
            None => None,
        }
    }
}


#[test]
pub fn test_it() {}