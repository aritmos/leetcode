use crate::Solution;

impl Solution {
    // no extra memory solution
    // pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
    //     let n = temps.len();
    //     let mut waits = vec![0; n];
    //
    //     for i in (0..(n - 1)).rev() {
    //         let mut j = i + 1;
    //         while waits[j] > 0 && temps[i] >= temps[j] {
    //             j += waits[j] as usize;
    //         }
    //         if temps[i] < temps[j] {
    //             waits[i] = (j - i) as i32
    //         }
    //     }
    //
    //     waits
    // }

    // stack solution
    pub fn daily_temperatures(temps: Vec<i32>) -> Vec<i32> {
        let n = temps.len();
        let mut waits = vec![0; n];
        let mut stack = Vec::new();
        for i in 0..n {
            while stack.last().map_or(false, |&j| temps[j] < temps[i]) {
                if let Some(j) = stack.pop() {
                    waits[j] = (i - j) as i32;
                }
                // Rust 1.58.2 version
                // stack.pop().map(|j| {
                //     waits[j] = (i - j) as i32;
                // });
            }
            stack.push(i);
        }

        waits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        let temps = vec![73, 74, 75, 71, 69, 72, 76, 73];
        let waits = vec![1, 1, 4, 2, 1, 1, 0, 0];
        assert_eq!(Solution::daily_temperatures(temps), waits);
    }
}
