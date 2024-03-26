// balanced binary tree
// TODO
use crate::structs::tree_node::*;
pub struct Solution;

use std::cmp::max;

type NodeRef = Rc<RefCell<TreeNode>>;

// impl Solution {
//     pub fn is_balanced(root: Option<NodeRef>) -> bool {
//         // returns Some(depth) of the node if balanced,
//         // if not balanced returns `None`
//         fn balanced_depth(root: Option<NodeRef>) -> Option<u32> {
//             let Some(inner_noderef) = root.clone() else {
//                 return Some(0);
//             };
//             let inner = inner_noderef.borrow();
//             let depth_l = balanced_depth(inner.left.clone())?;
//             let depth_r = balanced_depth(inner.right.clone())?;
//             if depth_l.abs_diff(depth_r) > 1 {
//                 None
//             } else {
//                 Some(1 + max(depth_l, depth_r))
//             }
//         }
//         balanced_depth(root).is_some()
//     }
// }

// Rust 1.58.2 Solution
impl Solution {
    pub fn is_balanced(root: Option<NodeRef>) -> bool {
        // returns Some(depth) of the node if balanced,
        // if not balanced returns `None`
        fn balanced_depth(root: Option<NodeRef>) -> Option<u32> {
            if let Some(inner_noderef) = root.clone() {
                let inner = inner_noderef.borrow();
                let depth_l = balanced_depth(inner.left.clone())?;
                let depth_r = balanced_depth(inner.right.clone())?;
                if (depth_l as i32 - depth_r as i32).abs() > 1 {
                    None
                } else {
                    Some(1 + max(depth_l, depth_r))
                }
            } else {
                Some(0)
            }
        }
        balanced_depth(root).is_some()
    }
}
