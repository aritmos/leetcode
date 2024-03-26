// 1. Two Sum
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen = std::collections::HashMap::new();
        for (i, val) in nums.iter().enumerate() {
            match seen.get(&(target - val)) {
                Some(&j) => return vec![i as i32, j as i32],
                None => seen.insert(val, i),
            };
        }
        unreachable!("Solution must exist!");
    }
}
