use crate::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::{BinaryHeap, HashMap};
        let mut counts = HashMap::<i32, u32>::new();
        for n in nums {
            *counts.entry(n).or_default() += 1;
        }

        let mut heap = BinaryHeap::new();
        for (x, c) in counts.into_iter() {
            heap.push((c, x));
        }

        let mut out = Vec::new();
        for _ in 0..k {
            out.push(heap.pop().unwrap().1);
        }

        out
    }
}
