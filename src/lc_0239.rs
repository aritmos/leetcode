use crate::Solution;

use std::collections::VecDeque;

struct MaxDeque<T> {
    deque: VecDeque<(usize, T)>,
}

impl MaxDeque<i32> {
    fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }

    /// (Possibly) adds the given element into its internal max deque.
    fn add(&mut self, idx: usize, n: i32) {
        let deque = &mut self.deque;
        if match deque.back() {
            Some(&(_, x)) => x >= n,
            None => true,
        } {
            deque.push_back((idx, n));
        } else {
            // clear all the elements smaller than `n`
            while let Some(&(_, last)) = deque.back() {
                if last < n {
                    deque.pop_back();
                } else {
                    break;
                }
            }
            deque.push_back((idx, n))
        }
    }

    /// Removes an element from the front if its index is less than or equal to `idx`
    fn remove_front(&mut self, idx: usize) {
        let deque = &mut self.deque;
        if let Some(&(front_idx, _)) = deque.front() {
            if front_idx <= idx {
                deque.pop_front();
            }
        }
    }
}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut maxdeque = MaxDeque::new();

        // initialise the deque state of the starting window
        nums.iter().enumerate().take(k).for_each(|(i, &n)| {
            maxdeque.add(i, n);
        });

        let starting_min = maxdeque.deque.front().unwrap_or(&(0, 0)).1;
        let mut max_vec = vec![starting_min];

        // iterate through the sliding windows
        nums.iter().enumerate().skip(k).for_each(|(i, &n)| {
            maxdeque.remove_front(i - k);
            maxdeque.add(i, n);

            max_vec.push(maxdeque.deque.front().unwrap().1);
        });

        max_vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3),
            vec![3, 3, 5, 5, 6, 7]
        );
        assert_eq!(Solution::max_sliding_window(vec![1, -1], 1), vec![1, -1]);
        assert_eq!(
            Solution::max_sliding_window(vec![1, 3, 1, 2, 0, 5], 3),
            vec![3, 3, 2, 5]
        );
    }
}
