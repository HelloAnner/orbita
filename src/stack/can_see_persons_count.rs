use crate::stack::Solution;

impl Solution {
    pub fn can_see_persons_count(heights: Vec<i32>) -> Vec<i32> {
        let n = heights.len();
        let mut ans = vec![0; n];
        let mut st = Vec::new();
        for (i, &h) in heights.iter().enumerate().rev() {
            while !st.is_empty() && *st.last().unwrap() < h {
                st.pop();
                ans[i] += 1;
            }
            if !st.is_empty() { // 还可以再看到一个人
                ans[i] += 1;
            }
            st.push(h);
        }
        ans
    }
}