use crate::Solution;

impl Solution {
    // pub fn trap(height: Vec<i32>) -> i32 {
    //     let mut trapped_area = 0;
    //     fn max_list(vals: &[i32], mut idxs: impl Iterator<Item = usize>) -> Vec<i32> {
    //         let mut out = vec![-1; vals.len()];
    //         let mut curr_max = vals[idxs.next().unwrap()];
    //         for i in idxs {
    //             curr_max = curr_max.max(vals[i]);
    //             out[i] = curr_max;
    //         }
    //         out
    //     }
    //     let max_l = max_list(&height, 0..height.len());
    //     let max_r = max_list(&height, (0..height.len()).rev());
    //
    //     println!("{max_l:?}");
    //     println!("{max_r:?}");
    //
    //     for i in 1..height.len() - 1 {
    //         let h = std::cmp::min(max_l[i], max_r[i]);
    //         println!("{h}");
    //         if h > height[i] {
    //             trapped_area += h - height[i]
    //         }
    //     }
    //     trapped_area
    // }

    // moving the smaller pointer works as it guarantees
    // the ability to calculate the min(max_l, max_r) with
    // only the two pointers, by transitive properties.
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut water_area = 0;
        let (mut l, mut r) = (0, height.len() - 1);
        let (mut max_l, mut max_r) = (height[l], height[r]);
        while l < r {
            if height[l] < height[r] {
                l += 1;
                max_l = max_l.max(height[l]);
                water_area += max_l - height[l];
            } else {
                r -= 1;
                max_r = max_r.max(height[r]);
                water_area += max_r - height[r];
            }
        }
        water_area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(Solution::trap(vec![4, 2, 3]), 1);
    }
}
