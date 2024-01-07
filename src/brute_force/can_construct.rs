use crate::brute_force::Solution;

// https://leetcode.cn/problems/ransom-note/?envType=daily-question&envId=2024-01-07
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let baseChar = 'a';
        let mut record = vec![0; 26];

        for c in magazine.bytes() {
            record[c as usize - baseChar as usize] += 1;
        }

        for c in ransom_note.bytes() {
            record[c as usize - baseChar as usize] -= 1;
            if (record[c as usize - baseChar as usize] < 0) {
                return false;
            }
        }

        true
    }
}