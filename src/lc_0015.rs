use crate::Solution;

// 3 sum
impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        let mut ans = vec![];
        nums.sort_unstable();
        for i in 0..nums.len() - 2 {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            if nums[i] > 0 {
                break;
            }
            let (mut j, mut k) = (i + 1, nums.len() - 1);
            while j < k {
                let sum = nums[i] + nums[j] + nums[k];
                match sum.cmp(&0) {
                    Ordering::Less => j += 1,
                    Ordering::Greater => k -= 1,
                    Ordering::Equal => {
                        ans.push(vec![nums[i], nums[j], nums[k]]);
                        while j < k && nums[j] == nums[j + 1] {
                            j += 1;
                        }
                        while j < k && nums[k] == nums[k - 1] {
                            k -= 1;
                        }
                    }
                }
            }
        }
        ans
    }
}
