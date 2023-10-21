use crate::Solution;

impl Solution {
    //! Product of Array Except Self
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefixes = nums;
        let mut suffixes = prefixes.clone();

        let mut curr_pre = 1;
        for v in prefixes.iter_mut() {
            curr_pre *= std::mem::replace(v, curr_pre);
        }

        let mut curr_post = 1;
        for v in suffixes.iter_mut().rev() {
            curr_post *= std::mem::replace(v, curr_post);
        }

        for (v1, v2) in prefixes.iter_mut().zip(suffixes) {
            *v1 *= v2;
        }

        prefixes
    }
}
