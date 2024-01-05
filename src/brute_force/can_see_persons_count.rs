use crate::brute_force::Solution;

// https://leetcode.cn/problems/number-of-visible-people-in-a-queue/
impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];

        for (i, h) in heights.iter().enumerate() {
            let mut count = 0;
            for j in (i + 1..heights.len()) {
                if heights[i] > heights[j] {
                    count += 1;
                } else {
                    break;
                }
            }
            ans.push(count);
        }
        ans
    }
}

#[test]
pub fn test_it() {
    // assert_eq!(vec![4, 1, 1, 1, 0], Solution::can_see_persons_count(vec![5, 1, 2, 3, 10]));
    // assert_eq!(vec![3, 1, 2, 1, 1, 0], Solution::can_see_persons_count(vec![10, 6, 8, 5, 11, 9]));
}