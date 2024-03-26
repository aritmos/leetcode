use crate::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (mut nums1, mut nums2) = (nums1, nums2);
        if nums1.len() > nums2.len() {
            std::mem::swap(&mut nums1, &mut nums2);
        }

        let (n, m) = (nums1.len(), nums2.len());
        let (mut lo, mut hi) = (0, n);
        let (mut i, mut j) = (0, 0);

        while lo <= hi {
            i = (lo + hi) / 2;
            j = (n + m + 1) / 2 - i;

            if (i < n && j > 0) && nums2[j - 1] > nums1[i] {
                lo = i + 1;
            } else if (i > 0 && j < m) && nums2[j] < nums1[i - 1] {
                hi = i - 1;
            } else {
                break;
            }
        }

        let median = if i == 0 {
            nums2[j - 1]
        } else if j == 0 {
            nums1[i - 1]
        } else {
            std::cmp::max(nums1[i - 1], nums2[j - 1])
        };

        let next = if (n + m) % 2 == 1 {
            median
        } else if i == n {
            nums2[j]
        } else if j == m {
            nums1[i]
        } else {
            std::cmp::min(nums1[i], nums2[j])
        };

        (median + next) as f64 / 2.0
    }
}

// min = 0, max = 7
//
// [2, 4, 7, 8, 10, 12, 20]        n = 7
//  ^  *  ^
// [1, 2, 3, 6, 6, 6, 8, 8, 9, 10] m = 10
//                          *
