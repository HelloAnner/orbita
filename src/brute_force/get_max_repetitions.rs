use crate::brute_force::Solution;

// https://leetcode.cn/problems/count-the-repetitions/
impl Solution {
    pub fn get_max_repetitions(s1: String, n1: i32, s2: String, n2: i32) -> i32 {
        let str1 = (0..n1).map(|_| s1).collect();
        let str2 = (0..n2).map(|_| s2).collect();

        -1
    }
}