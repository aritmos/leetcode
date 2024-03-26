use crate::Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut letter_freq = [0; 26];
        let mut max_freq = 0;
        let (mut l_idx, mut max_len) = (0, 0);
        let s_bytes = s.as_bytes();

        for (r_idx, &c) in s_bytes.iter().enumerate() {
            let idx = (c - b'A') as usize;
            letter_freq[idx] += 1;
            max_freq = max_freq.max(letter_freq[idx]);

            let len = r_idx + 1 - l_idx;
            if len - max_freq > k as usize {
                letter_freq[(s_bytes[l_idx] - b'A') as usize] -= 1;
                l_idx += 1;
            }
            max_len = max_len.max(len);
        }
        max_len as i32
    }
}

pub fn character_replacement(s: String, k: i32) -> i32 {
    let k = k as usize; // k should always be an unsigned integer given the problem statement
    if s.len() <= k + 1 {
        return s.len() as i32;
    }

    /// Returns the count of the character which appears most often in the string.
    /// Panics if it encounters a byte that is not an uppercase ascii letter.
    fn max_count(s: &str) -> usize {
        let mut counts = [0_usize; 26];
        for byte in s.as_bytes() {
            assert!(byte.is_ascii_uppercase());
            counts[(byte - b'A') as usize] += 1;
        }
        counts.into_iter().max().unwrap()
    }

    let (mut l, mut r) = (0, k + 1);
    while r <= s.len() {
        let window = &s[l..r];
        println!("({l}, {r}): {window}");
        let fill = window.len() - max_count(window);

        l += (fill > k) as usize;
        r += 1;
    }

    (r - l - 1) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(Solution::character_replacement("ABAB".to_owned(), 2), 4);
        assert_eq!(Solution::character_replacement("AABABBA".to_owned(), 1), 4);
    }
}
