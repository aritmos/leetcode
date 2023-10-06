/// Reverse Linked List
use crate::structs::list_node::*;
use crate::Solution;

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = head;
        let mut new_list: Option<Box<ListNode>> = None;
        while let Some(node) = list {
            list = node.next;
            new_list = Some(Box::new(ListNode {
                val: node.val,
                next: new_list,
            }));
        }
        new_list
    }
}
