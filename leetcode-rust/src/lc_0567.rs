use crate::Solution;

#[derive(PartialEq, Eq)]
struct LetterCounts {
    arr: [u32; 26],
}

impl LetterCounts {
    fn new() -> Self {
        Self { arr: [0; 26] }
    }

    fn add(&mut self, byte: u8) {
        self.arr[(byte - b'a') as usize] += 1;
    }

    fn remove(&mut self, byte: u8) {
        self.arr[(byte - b'a') as usize] -= 1;
    }

    fn from_str(s: &str) -> Self {
        s.as_bytes().iter().fold(Self::new(), |mut counts, &b| {
            counts.add(b);
            counts
        })
    }
}

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }

        let s1_counts = LetterCounts::from_str(&s1);
        let mut window_counts = LetterCounts::from_str(&s2[..s1.len()]);

        for l in 0.. {
            if s1_counts == window_counts {
                return true;
            }

            let r = l + s1.len();
            if r >= s2.len() {
                break;
            }

            window_counts.add(s2.as_bytes()[r]);
            window_counts.remove(s2.as_bytes()[l]);
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base1() {
        assert!(Solution::check_inclusion("ab".into(), "eidbaooo".into()));
    }

    #[test]
    fn base2() {
        assert!(!Solution::check_inclusion("ab".into(), "eidboaoo".into()));
    }

    #[test]
    fn cust1() {
        assert!(Solution::check_inclusion("ab".into(), "eidboab".into()));
    }
}
