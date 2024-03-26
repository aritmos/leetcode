use crate::structs::tree_node::*;
use crate::Solution;
use std::cmp::max;

type NodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<NodeRef>) -> i32 {
        /// Returns the trees depth (`None` => 0), and diameter.
        /// Updates `max_diameter_ref` if the tree's diameter is larger than the ref's value.
        fn dfs(node: &Option<NodeRef>, max_diameter_ref: &mut u32) -> Option<u32> {
            let Some(node_ref) = node else {
                return None;
            };
            let node = node_ref.borrow();

            let left_depth = dfs(&node.left, max_diameter_ref);
            let right_depth = dfs(&node.right, max_diameter_ref);

            // Add 1 for each edge and sum the depths
            // => max diameter in the current subtree
            let diameter = [left_depth, right_depth]
                .iter()
                .map(|opt_x| opt_x.map(|x| x + 1).unwrap_or(0))
                .sum();

            *max_diameter_ref = max(*max_diameter_ref, diameter);

            let depth = max(left_depth, right_depth).map(|x| x + 1).unwrap_or(0);
            Some(depth)
        }

        // we are forced to use an external counter as the calculation of a subtree's diameter
        // requires the depth of its subtrees, but in the final root call we also want
        // to add the connecting edges, hence the function cannot be pure recursion.
        let mut max_diameter = 0;
        dfs(&root, &mut max_diameter);
        max_diameter as i32
    }
}
