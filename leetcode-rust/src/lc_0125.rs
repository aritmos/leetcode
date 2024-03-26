// valid palindrome
pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut bytes = s
            .bytes()
            .filter_map(|b| b.is_ascii_alphanumeric().then_some(b.to_ascii_lowercase()));

        loop {
            if let (Some(a), Some(b)) = (bytes.next(), bytes.next_back()) {
                if a != b {
                    return false;
                }
            } else {
                return true;
            }
        }
    }
}
