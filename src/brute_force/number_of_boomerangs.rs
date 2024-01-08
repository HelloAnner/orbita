use std::collections::HashMap;

use crate::brute_force::Solution;

// https://leetcode.cn/problems/number-of-boomerangs/?envType=daily-question&envId=2024-01-08

impl Solution {
    pub fn number_of_boomerangs(points: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut cnt = HashMap::new();
        for p1 in &points {
            cnt.clear();
            for p2 in &points {
                let d = (p1[0] - p2[0]).pow(2) + (p1[1] - p2[1]).pow(2);
                let mut v = cnt.entry(d).or_insert(0);
                ans += *v * 2;
                *v += 1;
            }
        }
        ans
    }
}

#[test]
pub fn test_it() {}