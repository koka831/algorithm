// https://leetcode.com/problems/add-two-numbers/
#![allow(unused)]

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

type MaybeNode = Option<Box<ListNode>>;

struct Solution;
impl Solution {
    // complexity: O(max(n, m)) where n = l1.len(), m = l2.len()
    pub fn add_two_numbers(l1: MaybeNode, l2: MaybeNode) -> MaybeNode {
        Self::rec(&l1, &l2, 0)
    }

    fn rec(l1: &MaybeNode, l2: &MaybeNode, carry: i32) -> MaybeNode {
        if l1.is_none() && l2.is_none() && carry == 0 {
            return None;
        }

        let mut n1 = l1;
        let mut n2 = l2;
        let mut val = carry;
        if let Some(l) = l1 {
            val += l.val;
            n1 = &l.next;
        }
        if let Some(l) = l2 {
            val += l.val;
            n2 = &l.next;
        }

        let sum = ListNode {
            val: val % 10,
            next: Self::rec(n1, n2, val / 10),
        };
        Some(Box::new(sum))
    }
}

fn main() {}
