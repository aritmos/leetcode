// binary search
pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut lo = 0;
        let mut hi = nums.len() - 1;
        while lo <= hi {
            let mid: usize = (hi + lo) / 2;
            if nums[mid] == target {
                return mid as i32;
            }
            if nums[mid] < target || lo == hi || mid == 0 {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
        -1
    }
}

#[cfg(test)]
mod binary_search {
    use super::*;
    #[test]
    fn eg1() {
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 9);
        assert_eq!(result, 4);
    }
    #[test]
    fn eg2() {
        let result = Solution::search(vec![-1, 0, 3, 5, 9, 12], 2);
        assert_eq!(result, -1);
    }
    #[test]
    fn eg3() {
        let result = Solution::search(vec![5], -5);
        assert_eq!(result, -1);
    }
}
