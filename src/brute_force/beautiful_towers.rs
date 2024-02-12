use crate::brute_force::Solution;

impl Solution {
    pub fn maximum_sum_of_heights(max_heights: Vec<i32>) -> i64 {
        let n = max_heights.len();
        let mut res = 0i64;

        for i in 0..n {
            let mut ans = 0i64;
            let mut cur = max_heights[i] as i64;

            for j in (0..=i).rev() {
                cur = cur.min(max_heights[j] as i64);
                ans += cur;
            }

            cur = max_heights[i] as i64;

            for j in i + 1..n {
                cur = cur.min(max_heights[j] as i64);
                ans += cur;
            }

            res = res.max(ans);
        }

        res
    }
}

#[test]
pub fn test_it() {
    Solution::maximum_sum_of_heights(vec![1000000000, 1000000000, 1000000000]);
}
