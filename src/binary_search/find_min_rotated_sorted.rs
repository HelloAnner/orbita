use crate::binary_search::origin::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let target = nums[nums.len() - 1];
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                right = mid - 1;
            } else if nums[mid as usize] > target {
                left = mid + 1;
            } else {}
        };
        -1
    }
}