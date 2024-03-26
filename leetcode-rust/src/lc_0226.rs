use crate::structs::tree_node::*;
use crate::Solution;

type TreeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn invert_tree(root: Option<TreeRef>) -> Option<TreeRef> {
        let tree_ref = root?;
        {
            let mut node = tree_ref.borrow_mut();

            let left = node.left.clone();
            node.left = Self::invert_tree(node.right.clone());
            node.right = Self::invert_tree(left);
        }
        Some(tree_ref)
    }
}
