use crate::structs::tree_node::*;
use crate::Solution;

type TreeRef = Rc<RefCell<TreeNode>>;

/// Maximum Depth of a Binary Tree
impl Solution {
    pub fn max_depth(root: Option<TreeRef>) -> i32 {
        let Some(node_ref) = root else {
            return 0;
        };
        let node = node_ref.borrow();
        let left_depth = Self::max_depth(node.left.clone());
        let right_depth = Self::max_depth(node.right.clone());

        std::cmp::max(left_depth, right_depth) + 1
    }
}
