// merge two sorted lists
use crate::structs::list_node::*;
use crate::Solution;

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (Some(l1), Some(l2)) => {
                if l1.val <= l2.val {
                    Some(Box::new(ListNode {
                        val: l1.val,
                        next: Self::merge_two_lists(l1.next, Some(l2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: l2.val,
                        next: Self::merge_two_lists(Some(l1), l2.next),
                    }))
                }
            }
            (None, Some(l2)) => Some(l2),
            (Some(l1), None) => Some(l1),
            (None, None) => None,
        }
    }
}

// smart solution
type Link = Option<Box<ListNode>>;

impl Solution {
    pub fn merge_two_lists_v2(mut l1: Link, mut l2: Link) -> Link {
        let mut r = &mut l1;
        while l2.is_some() {
            if r.is_none() || l2.as_ref()?.val < r.as_ref()?.val {
                std::mem::swap(r, &mut l2);
            }
            r = &mut r.as_mut()?.next;
        }
        l1
    }
}
