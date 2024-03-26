use crate::Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut nums = nums;
        nums.sort_unstable();

        let mut max_count = 1;
        let mut cur_count = 1;
        let mut last_num = nums[0]; // dummy value
        for num in &nums[1..] {
            match num - last_num {
                0 => (),
                1 => cur_count += 1,
                2.. => {
                    max_count = i32::max(max_count, cur_count);
                    cur_count = 1
                }
                _ => unreachable!(),
            }
            last_num = *num;
        }

        i32::max(max_count, cur_count)
    }
}
