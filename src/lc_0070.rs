// climbing stairs
pub struct Solution;

impl Solution {
    // basically just fibonacci
    pub fn climb_stairs(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;

        for _ in 2..=n {
            let temp = a + b;
            a = b;
            b = temp;
        }
        b
    }
}

#[cfg(test)]
mod test_0070 {
    use super::*;

    #[test]
    fn eg() {
        println!("{}", Solution::climb_stairs(2));
    }
}
