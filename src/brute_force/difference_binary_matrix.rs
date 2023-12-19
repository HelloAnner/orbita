use crate::brute_force::Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut rows = vec![0; grid.len()];
        let mut cols = vec![0; grid[0].len()];
        let mut ans = vec![vec![0; grid[0].len()]; grid.len()];

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                rows[r] += grid[r][c];
                cols[c] += grid[r][c];
            }
        }

        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                ans[r][c] = rows[r] - (grid.len() as i32 - rows[r]) + cols[c] - (grid[0].len() as i32 - cols[c]);
            }
        }

        ans
    }
}

#[test]
pub fn test_it() {}