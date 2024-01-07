use crate::linked_list::{ListNode, Solution};

// https://leetcode.cn/problems/insert-greatest-common-divisors-in-linked-list/?envType=daily-question&envId=2024-01-06
impl Solution {
    pub fn insert_greatest_common_divisors(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;

        while cur.as_ref().unwrap().next.is_some() {
            let x = cur.as_mut().unwrap();
            let next = x.next.take();
            x.next = Some(Box::new(ListNode {
                val: Self::gcd(x.val, next.as_ref().unwrap().val),
                next,
            }));
            cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
        }

        head
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while a != 0 {
            (a, b) = (b % a, a);
        }
        b
    }
}