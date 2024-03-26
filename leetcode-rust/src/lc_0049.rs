use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;
        type Count = [u16; 26];
        let mut anagrams: HashMap<Count, Vec<String>> = HashMap::new();

        #[inline]
        fn count(s: &str) -> Count {
            let mut count = [0; 26];
            for b in s.bytes() {
                count[(b - b'a') as usize] += 1;
            }
            count
        }

        for word in strs {
            let count = count(&word);
            anagrams.entry(count).or_default().push(word);
        }

        anagrams.into_values().collect()
    }
}
