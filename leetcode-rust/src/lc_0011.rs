use crate::Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, height.len() - 1);
        let mut max_area = 0;
        while l < r {
            let hl = height[l];
            let hr = height[r];
            max_area = max_area.max(std::cmp::min(hl, hr) * (r - l) as i32);
            match hl.cmp(&hr) {
                std::cmp::Ordering::Less => l += 1,
                std::cmp::Ordering::Greater => r -= 1,
                std::cmp::Ordering::Equal => {
                    l += 1;
                    r -= 1;
                }
            }
        }
        max_area
    }
}
