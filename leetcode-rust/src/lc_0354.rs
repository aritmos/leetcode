use crate::Solution;

impl Solution {
    pub fn max_envelopes(envelopes: Vec<[i32; 2]>) -> i32 {
        let mut envelopes = envelopes;
        envelopes.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));

        // longest increasing subsequence (binary search version)
        let mut lis = Vec::new();
        for envelope in envelopes {
            let height = envelope[1];
            if height > *lis.last().unwrap() {
                lis.push(height);
            } else {
                let idx = match lis.binary_search(&height) {
                    Ok(idx) => idx,
                    Err(idx) => idx,
                };

                lis[idx] = height;
            }
        }

        lis.len() as i32
    }
}
