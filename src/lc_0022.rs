use crate::Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut stack = Vec::new();
        let mut out = Vec::new();

        fn backtrack(n: u32, open: u32, close: u32, stack: &mut Vec<u8>, out: &mut Vec<String>) {
            if open == n && close == n {
                return out.push(unsafe { String::from_utf8_unchecked(stack.clone()) });
            }
            if open < n {
                stack.push(b'(');
                backtrack(n, open + 1, close, stack, out);
                stack.pop();
            }
            if close < open {
                stack.push(b')');
                backtrack(n, open, close + 1, stack, out);
                stack.pop();
            }
        }

        backtrack(n as u32, 0, 0, &mut stack, &mut out);
        out
    }
}
