use crate::binary_search::origin::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left + 1 < right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid;
            } else if nums[mid as usize] > target {
                right = mid;
            } else {
                // 其实就是在不断向左逼近
                right = mid;
            }
        }

        // 手动找 first index 或者 last index
        if nums[left as usize] == target {
            return left;
        }

        if nums[right as usize] == target {
            return right;
        }

        -1
    }
}

#[test]
pub fn test_it() {}
