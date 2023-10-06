use crate::Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.as_bytes() {
            match c {
                b'(' | b'[' | b'{' => stack.push(c),
                _ => {
                    let Some(open) = stack.pop() else {
                        return false;
                    };
                    if (c - open) > 2 {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}
