use crate::Solution;

impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        fn idx_pass(vals: &[i32], range: impl IntoIterator<Item = usize>) -> Vec<i32> {
            let n = vals.len();
            let mut out = vec![-1; n];
            let mut stack = Vec::new();
            for i in range {
                let curr_val = vals[i];
                while stack
                    .last()
                    .map_or(false, |&v: &(usize, i32)| curr_val < v.1)
                {
                    let (idx, _) = stack.pop().unwrap();
                    out[idx] = i as i32;
                }
                stack.push((i, curr_val));
            }
            out
        }
        let n = heights.len();
        let prev_idxs = idx_pass(&heights, (0..n).rev());
        let next_idxs = idx_pass(&heights, 0..n);
        let mut max_area = 0;

        for i in 0..n {
            let left = prev_idxs[i];
            let right = match next_idxs[i] {
                -1 => n as i32,
                x => x,
            };

            let area = heights[i] * (right - left - 1);
            max_area = std::cmp::max(max_area, area);
        }
        max_area
    }
}

// h: [2, 4]
// l: [n, 0] -> [0, 0]
// r: [n, n] -> [1, 1]
