use std::cmp::Ordering;

use crate::brute_force::Solution;

// https://leetcode.cn/problems/determine-the-winner-of-a-bowling-game/?envType=daily-question&envId=2023-12-27
impl Solution {
    pub fn is_winner(player1: Vec<i32>, player2: Vec<i32>) -> i32 {
        let s1 = Self::score(&player1);
        let s2 = Self::score(&player2);

        match s1.cmp(&s2) {
            Ordering::Less => 2,
            Ordering::Greater => 1,
            Ordering::Equal => 0,
        }
    }

    fn score(player: &Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut need_more = 0;

        player.iter().for_each(|score| {
            if need_more > 0 {
                ans += 2 * *score;
                need_more -= 1;
            } else {
                ans += *score;
            }

            if *score == 10 {
                need_more = 2;
            }
        });

        ans
    }
}

#[test]
pub fn test_it() {
    assert_eq!(1, Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]));
    assert_eq!(2, Solution::is_winner(vec![3, 6, 10, 8], vec![9, 9, 9, 9]));
}