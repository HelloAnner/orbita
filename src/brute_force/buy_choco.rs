use crate::brute_force::Solution;

impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        // 维护一个最小的元素和次小的元素
        let mut first = i32::MAX;
        let mut second = i32::MAX;
        for p in prices {
            if first > p {
                second = first;
                first = p;
            } else if second > p {
                second = p;
            }
        }

        if money >= first + second {
            return money - first - second;
        }

        money
    }
}

#[test]
pub fn test_it() {
    assert_eq!(0, Solution::buy_choco(vec![1, 2, 2], 3))
}