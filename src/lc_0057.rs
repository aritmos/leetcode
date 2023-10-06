/// Insert Interval
use crate::Solution;

impl Solution {
    /// An interval should be represented as `[i32; 2]` or `RangeInclusive<i32>`, etc.
    /// This would greatly optimize the algorithm, as the inner `Vec<i32>` is not Copy.
    /// With a reasonable data structure one can simply calculate the indices,
    /// overwrite (or in the worst case insert) the (possibly modified) `new_interval`
    /// into the Vec and then use `copy_from_within` + `resize`.
    /// If `new_interval` overlaps with any interval then this results in no new allocations.
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let (mut s, mut e) = (new_interval[0], new_interval[1]);
        let li = intervals
            .binary_search_by_key(&s, |v| v[0])
            .unwrap_or_else(|x| x);
        let ri = intervals
            .binary_search_by_key(&e, |v| v[1])
            .unwrap_or_else(|x| x);

        let l = &intervals[..(li + usize::from(li < intervals.len() && intervals[li][1] < s))];
        let r = &intervals[(ri + usize::from(ri >= intervals.len() || intervals[ri][0] <= e))
            .min(intervals.len())..];
        if l.len() + r.len() != intervals.len() {
            s = s.min(intervals[l.len()][0]);
            e = e.max(intervals[intervals.len() - r.len() - 1][1]);
        }
        // unavoidable allocation occurs here due to poor choice of data structure
        [l, &[vec![s, e]], r].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_idx() {
        let intervals = vec![vec![1, 3], vec![3, 6], vec![8, 10]];
        let n = locate(&intervals, 0, 2);
        assert_eq!(n, 1);
    }

    #[test]
    fn end_idx() {
        let intervals = vec![vec![1, 3], vec![3, 6], vec![8, 10]];
        let n = locate(&intervals, 1, 8);
        assert_eq!(n, 2);
    }

    #[test]
    fn equal_start() {
        let intervals = vec![vec![1, 3], vec![3, 6], vec![8, 10]];
        let n = locate(&intervals, 0, 3);
        assert_eq!(n, 1);
    }

    #[test]
    fn equal_end() {
        let intervals = vec![vec![1, 3], vec![3, 6], vec![8, 10]];
        let n = locate(&intervals, 0, 6);
        assert_eq!(n, 3);
    }

    #[test]
    fn within_range() {
        let intervals = vec![vec![1, 3], vec![3, 6], vec![8, 10]];
        let new_interval = vec![2, 7];
        assert_eq!(Solution::insert(intervals, new_interval), (true, false));
    }
}
