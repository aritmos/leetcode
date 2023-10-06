use crate::structs::tree_node::*;
/// Maximum Depth of a Binary Tree
use crate::Solution;

type TreeRef = Rc<RefCell<TreeNode>>;
impl Solution {
    pub fn max_depth(root: Option<TreeRef>) -> i32 {
        fn depth(node: Option<TreeRef>) -> u32 {
            let Some(node_ref) = node else {
                return 0;
            };
            let node = node_ref.borrow();
            let left_depth = depth(node.left.clone());
            let right_depth = depth(node.right.clone());

            std::cmp::max(left_depth, right_depth) + 1
        }
        depth(root) as i32
    }
}
