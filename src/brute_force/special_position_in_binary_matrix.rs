use crate::brute_force::Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for r in 0..mat.len() {
            let c = Self::check_row(&mat, r);
            if c >= 0 && Self::check_col(&mat, r, c as usize) {
                ans += 1;
            }
        }
        ans
    }

    fn check_row(mat: &Vec<Vec<i32>>, index: usize) -> i32 {
        let mut ans = -1_i32;
        for c in 0..mat[0].len() {
            if mat[index][c] == 1 {
                if ans >= 0 {
                    return -1;
                } else {
                    ans = c as i32;
                }
            }
        }
        ans
    }

    fn check_col(mat: &Vec<Vec<i32>>, current_row: usize, current_col: usize) -> bool {
        for r in 0..mat.len() {
            if mat[r][current_col] == 1 && r != current_row {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_it() {}