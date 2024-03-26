// maximum subarray
// TODO
use crate::Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut max_sum = nums[0];
        for n in nums {
            sum = i32::max(n, sum + n);
            if sum > max_sum {
                max_sum = sum;
            }
        }
        max_sum
    }
}
