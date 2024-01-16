use crate::linked_list::{ListNode, Solution};

//  https://leetcode.cn/problems/remove-duplicates-from-sorted-list-ii/?envType=daily-question&envId=2024-01-15

// 删除排序链表中的重复元素


impl Solution {
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        dummy.next = head;
        let mut cur = &mut dummy;
        while let Some(next) = cur.next.as_mut() {
            if let Some(next_next) = next.next.as_mut() {
                let val = next.val;
                if next_next.val == val {
                    while let Some(node) = cur.next.as_mut() {
                        if node.val == val {
                            cur.next = node.next.take();
                        } else {
                            break;
                        }
                    }
                } else {
                    cur = cur.next.as_mut().unwrap();
                }
            } else {
                break;
            }
        }
        dummy.next
    }
}