use crate::Solution;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        const LEN: usize = (b'z' - b'A' + 1) as usize;
        let mut count = {
            let mut arr = [0_i32; LEN];
            for b in t.as_bytes() {
                arr[(b - b'A') as usize] -= 1;
            }
            arr
        };

        fn all_non_neg(slice: &[i32]) -> bool {
            slice.iter().all(|&x| x >= 0)
        }

        // Find the first working substring
        let mut l_idx = 0;
        let Some(mut r_idx) = s.as_bytes().iter().position(|b| {
            count[(b - b'A') as usize] += 1;
            all_non_neg(&count)
        }) else {
            return "".to_string();
        };

        let (mut best_l_idx, mut best_r_idx) = (l_idx, r_idx);

        // Go through the string to find the minimum substring,
        // starting with our initial working substring

        loop {
            let b = s.as_bytes()[l_idx];
            count[(b - b'A') as usize] -= 1;
            l_idx += 1;

            if all_non_neg(&count) {
                best_l_idx = l_idx;
                best_r_idx = r_idx;
            } else {
                r_idx += 1;
                if r_idx == s.len() {
                    break;
                }
                let b = s.as_bytes()[r_idx];
                count[(b - b'A') as usize] += 1;
            }
        }

        s[best_l_idx..best_r_idx + 1].to_string()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic() {
        let s = String::from("ADOBECODEBANC");
        let t = String::from("ABC");
        assert_eq!(Solution::min_window(s, t), "BANC");
    }
}
