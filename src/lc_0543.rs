use crate::structs::tree_node::*;
use crate::Solution;
use std::cmp::max;

type NodeRef = Rc<RefCell<TreeNode>>;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<NodeRef>) -> i32 {
        let mut max_diameter = 0;
        fn dfs(node: Option<NodeRef>, max_diameter_ref: &mut u32) -> u32 {
            let Some(inner_noderef) = node else {
                return 0;
            };
            let inner = inner_noderef.borrow();
            let left_depth = dfs(inner.left.clone(), max_diameter_ref);
            let right_depth = dfs(inner.right.clone(), max_diameter_ref);

            let diameter = left_depth + right_depth + 2; // max diameter in the current subtree
            *max_diameter_ref = max(*max_diameter_ref, diameter);

            max(left_depth, right_depth)
        }
        dfs(root, &mut max_diameter);
        max_diameter as i32
    }
}

// Rust 1.58.2 Solution
impl Solution {
    pub fn diameter_of_binary_tree_rv58(root: Option<NodeRef>) -> i32 {
        let mut max_diameter = 0;
        fn dfs(node: Option<NodeRef>, max_diameter: &mut u32) -> Option<u32> {
            node.as_ref()?; // return None if the node is None
            let inner_noderef = node.unwrap();
            let inner = inner_noderef.borrow();

            let left_depth = dfs(inner.left.clone(), max_diameter);
            let right_depth = dfs(inner.right.clone(), max_diameter);

            // each Some result will add an edge to our diameter calculation
            let edges = [left_depth, right_depth]
                .iter()
                .filter(|x| x.is_some())
                .count() as u32;
            if edges == 0 {
                return Some(0); // we are at a leaf node
            }

            let left = left_depth.unwrap_or(0);
            let right = right_depth.unwrap_or(0);

            let diameter = left + right + edges; // diameter of the current node
            *max_diameter = max(*max_diameter, diameter);

            Some(max(left, right) + 1)
        }
        dfs(root, &mut max_diameter);
        max_diameter as i32
    }
}
