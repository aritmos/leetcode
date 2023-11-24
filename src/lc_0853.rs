use crate::Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut pos_and_times = Iterator::zip(position.iter(), speed)
            .map(|(&p, s)| (p, (target - p) as f64 / s as f64))
            .collect::<Vec<_>>();
        pos_and_times.sort_unstable_by_key(|x| x.0);

        let mut fleets = 0;
        let mut dest_time = 0.0;
        while let Some((_, time)) = pos_and_times.pop() {
            if time > dest_time {
                dest_time = time;
                fleets += 1;
            }
        }
        fleets
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base() {
        assert_eq!(
            Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
            3
        )
    }
}
