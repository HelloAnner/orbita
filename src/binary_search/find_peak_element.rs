// https://leetcode.cn/problems/find-peak-element/solutions/998152/xun-zhao-feng-zhi-by-leetcode-solution-96sj/?envType=daily-question&envId=2023-12-19
// 峰值元素是指其值严格大于左右相邻值的元素。
// 给你一个整数数组 nums，找到峰值元素并返回其索引。数组可能包含多个峰值，在这种情况下，返回 任何一个峰值 所在位置即可。
// 你可以假设 nums[-1] = nums[n] = -∞ 。
// 你必须实现时间复杂度为 O(log n) 的算法来解决此问题。

use crate::binary_search::origin::Solution;

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if mid < nums.len() - 1 && nums[mid] < nums[mid + 1] {
                left = mid + 1;
            } else if mid > 0 && nums[mid] < nums[mid - 1] {
                right = mid - 1;
            } else {
                left = mid;
                break;
            }
        }

        left as i32
    }
}
