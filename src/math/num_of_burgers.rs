// https://leetcode.cn/problems/number-of-burgers-with-no-waste-of-ingredients/description/?envType=daily-question&envId=2023-12-25

use crate::math::Solution;

impl Solution {
    pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
        if (tomato_slices % 2 != 0 || tomato_slices < 2 * cheese_slices || 4 * cheese_slices < tomato_slices) {
            return vec![];
        }
        vec![tomato_slices / 2 - cheese_slices, 2 * cheese_slices - tomato_slices / 2]
    }
}

#[test]
pub fn test_it() {
    assert_eq!(vec![1, 6], Solution::num_of_burgers(16, 7))
}