// lowest common ancestor of a bst
pub struct Solution;

use crate::structs::tree_node::*;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let x = p.unwrap().borrow().val;
        let y = q.unwrap().borrow().val;
        let mut curr = root;
        while let Some(inner) = curr.clone() {
            let inner_ref = inner.borrow();
            let curr_val = inner_ref.val;
            if x < curr_val && y < curr_val {
                curr = inner_ref.left.clone();
            } else if curr_val < x && curr_val < y {
                curr = inner_ref.right.clone();
            } else {
                // must have x <= root_val <= y
                return curr;
            }
        }
        None
    }
}
