pub struct Solution;

impl Solution {
    // @param nums: The integer array.
    // @param target: Target to find.
    // @return: The first position of target. Position starts from 0.
    pub fn binary_search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = nums.len();
        while left < right {
            let mid = (left + right) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else if nums[mid] > target {
                right = mid;
            } else {
                return mid as i32;
            }
        }
        -1
    }
}
