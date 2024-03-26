use crate::structs::tree_node::*;
use crate::Solution;

use std::cell::RefCell;
use std::rc::Rc;

type NodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn is_valid_bst(root: Option<NodeRef>) -> bool {
        // we use optionals to account for the cases of nodes with i32::MIN and i32::MAX values,
        // as passing these into lo and hi would
        fn is_valid_bst_inner(node: Option<NodeRef>, lo: Option<i32>, hi: Option<i32>) -> bool {
            // nothing to do if empty
            if node.is_none() {
                return true;
            }

            // check that val lies within bounds
            let node = node.unwrap();
            let val = node.borrow().val;

            let lo_ok = match lo {
                Some(lo) => lo < val,
                None => true,
            };
            let hi_ok = match hi {
                Some(hi) => val < hi,
                None => true,
            };
            if !(lo_ok && hi_ok) {
                return false;
            }

            // recursively check nodes
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            let left_branch_ok = is_valid_bst_inner(left, lo, Some(val));
            let right_branch_ok = is_valid_bst_inner(right, Some(val), hi);

            left_branch_ok && right_branch_ok
        }

        is_valid_bst_inner(root, None, None)
    }
}
