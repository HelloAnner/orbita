// https://leetcode.cn/problems/check-if-a-string-is-an-acronym-of-words/?envType=daily-question&envId=2023-12-20

// 给你一个字符串数组 words 和一个字符串 s ，请你判断 s 是不是 words 的 首字母缩略词 。
// 如果可以按顺序串联 words 中每个字符串的第一个字符形成字符串 s ，则认为 s 是 words 的首字母缩略词。例如，"ab" 可以由 ["apple", "banana"] 形成，但是无法从 ["bear", "aardvark"] 形成。
// 如果 s 是 words 的首字母缩略词，返回 true ；否则，返回 false 。

use crate::brute_force::Solution;

impl Solution {
    pub fn is_acronym(words: Vec<String>, s: String) -> bool {
        let mut ss = String::new();

        for w in words {
            ss.push(w.chars().next().unwrap())
        }

        ss.eq(&s)
    }
}

#[test]
fn test_it() {
    assert_eq!(true, Solution::is_acronym(vec!["as".to_string(), "uu".to_string()], "au".to_string()));
    assert_eq!(false, Solution::is_acronym(vec!["as".to_string(), "uu".to_string()], "bu".to_string()));
}