use crate::binary_search::origin::Solution;

impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut ans = x as i128;
        while ans * ans > x as i128 {
            ans = (ans + x as i128 / ans) / 2;
        }
        ans as i32
    }
}
