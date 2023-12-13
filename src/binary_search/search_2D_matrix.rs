use crate::binary_search::origin::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let row_size = matrix.len();
        let col_size = matrix[0].len();

        let mut r = row_size as i32 - 1;
        let mut c = 0_i32;

        while r >= 0 && c < col_size as i32 {
            // 最后一层，最左侧
            if matrix[r as usize][c as usize] < target {
                c += 1;
            } else if matrix[r as usize][c as usize] > target {
                r -= 1;
            } else {
                c += 1;
                r -= 1;
                return true;
            }
        }

        false
    }
}

#[test]
pub fn test_it() {
    assert_eq!(false, Solution::search_matrix(vec![vec![-5]], -10))
}