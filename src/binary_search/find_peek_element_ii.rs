use crate::binary_search::origin::Solution;

// 一个 2D 网格中的 峰值 是指那些 严格大于 其相邻格子(上、下、左、右)的元素。
// 给你一个 从 0 开始编号 的 m x n 矩阵 mat ，其中任意两个相邻格子的值都 不相同 。找出 任意一个 峰值 mat[i][j] 并 返回其位置 [i,j] 。
// 你可以假设整个矩阵周边环绕着一圈值为 -1 的格子。
// 要求必须写出时间复杂度为 O(m log(n)) 或 O(n log(m)) 的算法


impl Solution {
    pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut left = 0;
        let mut right = mat.len() - 1;

        while left < right {
            let mid = left + (right - left) / 2;
            let max_col_index = mat[mid].iter().position(|&x| x == *mat[mid].iter().max().unwrap()).unwrap();

            if mat[mid][max_col_index] > mat[mid + 1][max_col_index] {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        let max_val = *mat[left].iter().max().unwrap();
        let max_index = mat[left].iter().position(|&x| x == max_val).unwrap();

        vec![left as i32, max_index as i32]
    }
}

#[test]
fn test_it() {}