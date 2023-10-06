// public dishes
#![allow(unused)]
pub struct Solution;

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut s = satisfaction;
        s.sort();
        let (mut sum, mut ans) = (0, 0);
        for i in (0..s.len()).rev() {
            if sum + s[i] > 0 {
                sum += s[i];
                ans += sum;
            } else {
                break;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test_1402 {

    use super::*;

    #[test]
    fn eg_1() {
        assert_eq!(14, Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]));
    }

    #[test]
    fn eg_2() {
        assert_eq!(20, Solution::max_satisfaction(vec![4, 3, 2]));
    }

    #[test]
    fn eg_3() {
        assert_eq!(0, Solution::max_satisfaction(vec![-1, -4, -5]));
    }
}
