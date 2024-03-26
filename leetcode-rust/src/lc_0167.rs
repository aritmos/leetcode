use crate::Solution;
use std::cmp::Ordering;

impl Solution {
    // Return type should really be a `(usize, usize)`
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0, numbers.len() - 1);
        loop {
            let sum = numbers[l] + numbers[r];
            match sum.cmp(&target) {
                Ordering::Less => l += 1,
                Ordering::Equal => break,
                Ordering::Greater => r -= 1,
            }
        }
        // array is 1 indexed
        vec![l as i32 + 1, r as i32 + 1]
    }
}
