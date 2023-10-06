// valid anagram
pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }

        let mut map = std::collections::HashMap::<char, usize>::with_capacity(s.len());
        for c in s.chars() {
            map.entry(c).and_modify(|n| *n += 1).or_insert(1);
        }

        for c in t.chars() {
            if let Some(count) = map.get_mut(&c) {
                if *count == 1 {
                    map.remove(&c);
                    continue;
                } else {
                    *count -= 1;
                }
            } else {
                return false;
            }
        }
        map.is_empty()
    }
}
