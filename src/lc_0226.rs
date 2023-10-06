use crate::structs::tree_node::*;
use crate::Solution;

type TreeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn invert_tree(_root: Option<TreeRef>) -> Option<TreeRef> {
        // let tree_ref = root?;
        // let TreeNode {
        //     ref mut left,
        //     ref mut right,
        //     ..
        // } = *tree_ref.borrow_mut();
        // std::mem::swap(left, right);
        // *left = Self::invert_tree(*left);
        //
        // Some(tree_ref)
        todo!()
    }
}
