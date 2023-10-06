// longest palindrome
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut counts: u64 = 0;
        let length = s.len();
        s.bytes().for_each(|b| counts ^= 1 << (b - 65));
        (length - counts.count_ones().saturating_sub(1) as usize) as i32
    }
}
