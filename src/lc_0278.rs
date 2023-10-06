// first bad version
// TODO
use crate::Solution;

// The API isBadVersion is defined for you.
// isBadVersion(version:i32)-> bool;
// to call it use self.isBadVersion(version)

impl Solution {
    pub fn is_bad_version(n: i32, b: i32) -> bool {
        n >= b
    }

    pub fn first_bad_version(n: i32, b: i32) -> i32 {
        let (mut lo, mut hi) = (0, n - 1);
        let mut i = 0;
        while hi > lo && i < 40 {
            let mid = (hi - lo) / 2 + lo;
            println!("> {lo} {mid} {hi}");
            if Self::is_bad_version(mid + 1, b) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
            i += 1;
        }
        lo + 1 // index versioning starts at 1
    }
}
// [n n y y y y]
//

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn basic() {
        assert_eq!(Solution::first_bad_version(1, 1), 1);
        assert_eq!(Solution::first_bad_version(5, 4), 4);
    }

    #[test]
    fn big() {
        assert_eq!(Solution::first_bad_version(123487234, 423567), 423567);
    }
}
