use crate::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut lo, mut hi) = (0, nums.len() - 1);

        while nums[lo] > nums[hi] {
            let mid = lo + (hi - lo) / 2;
            match nums[lo] > nums[mid] {
                true => hi = mid,
                false => lo = mid + 1,
            }
        }

        nums[lo]
    }
}

// we always have L > R,
// if we don't then L is the minimum element
//
// [4, 5, 6, 7, 0, 1, 2]
//              ^
//                    ^
//
// [3, 1, 2]
//  ^
//  *  ^
//
// [2, 1]
//  ^
//     ^

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![3, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![2, 1]), 1);
    }
}
