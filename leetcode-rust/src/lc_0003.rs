// longest substring without repeating characters
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut start_index = 0;
        let mut max_length = 0;
        let mut char_indices = std::collections::HashMap::new();
        for (i, c) in s.chars().enumerate() {
            if let Some(&char_index) = char_indices.get(&c) {
                if char_index >= start_index {
                    start_index = char_index + 1;
                }
            }
            let length = i - start_index + 1;
            if length > max_length {
                max_length = length;
            }
            char_indices.insert(c, i);
        }
        max_length as i32
    }
}
