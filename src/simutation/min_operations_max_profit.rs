// https://leetcode.cn/problems/maximum-profit-of-operating-a-centennial-wheel/?envType=daily-question&envId=2024-01-01

use std::cmp::min;

use crate::simutation::Solution;

impl Solution {
    pub fn min_operations_max_profit(customers: Vec<i32>, boarding_cost: i32, running_cost: i32) -> i32 {
        let mut ans = -1;
        let mut wait = 0;
        let mut index = 0;
        let mut profit = 0;
        let mut max_profit = 0;

        while wait > 0 || index < customers.len() {
            wait += if index < customers.len() { customers[index] } else { 0 };
            let up = min(4, wait);
            wait -= up;
            index += 1;
            profit += up * boarding_cost - running_cost;

            if profit > max_profit {
                max_profit = profit;
                ans = index as i32;
            }
        }

        ans
    }
}