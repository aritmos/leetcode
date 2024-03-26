/// Contains Duplicate
use crate::Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // alternatively we can use `with_capacity(nums.len())`
        // if we don't care about overallocating
        let mut seen = std::collections::HashSet::new();
        nums.iter().any(|n| !seen.insert(n))
    }
}
