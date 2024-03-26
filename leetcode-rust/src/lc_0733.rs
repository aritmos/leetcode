// flood fill
pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        if image[sr as usize][sc as usize] == color {
            return image;
        }
        let (m, n) = (image.len(), image[0].len());
        let initial = image[sr as usize][sc as usize];
        let mut stack = vec![(sr, sc)];
        while let Some((x, y)) = stack.pop() {
            image[x as usize][y as usize] = color;
            for (dx, dy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let (xx, yy) = (x + dx, y + dy);
                if 0 <= xx
                    && xx < m as i32
                    && 0 <= yy
                    && yy < n as i32
                    && image[xx as usize][yy as usize] == initial
                {
                    stack.push((xx, yy))
                }
            }
        }
        image
    }
}

#[cfg(test)]
mod flood_fill {
    use super::*;

    #[test]
    fn eg1() {
        let ans = Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2);
        assert_eq!(ans, vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]);
    }
}
