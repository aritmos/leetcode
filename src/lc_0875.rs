use crate::Solution;

impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        // both inputs should have u32 instead of i32 given the context
        // we explicitly cast them to be correct
        let piles: Vec<u32> = unsafe { std::mem::transmute(piles) };
        let h = h as u32;

        // the minimum amount of time is: piles.sum() / h
        // the maximum amount of time is: piles.max()
        // we do a binary search to find which is the lowest value that works
        let mut min_rate = piles.iter().sum::<u32>().div_ceil(h);
        let mut max_rate = *piles.iter().max().unwrap() as u32;

        while min_rate < max_rate {
            let k = (max_rate + min_rate) / 2;
            let is_rate_ok = piles.iter().map(|&x| x.div_ceil(k)).sum::<u32>() <= h;
            match is_rate_ok {
                true => max_rate = k,
                false => min_rate = k + 1,
            }
        }

        min_rate as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let (piles, hours) = (vec![3, 6, 7, 11], 8);
        assert_eq!(Solution::min_eating_speed(piles, hours), 4);

        let (piles, hours) = (vec![30, 11, 23, 4, 20], 5);
        assert_eq!(Solution::min_eating_speed(piles, hours), 30);
    }
}
