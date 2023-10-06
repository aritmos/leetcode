// majority element
use crate::Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut majority_element = 0;
        for n in nums {
            if count == 0 {
                majority_element = n;
            }
            if majority_element == n {
                count += 1;
            } else {
                count -= 1;
            }
        }
        majority_element
    }
}
