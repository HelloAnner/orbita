use crate::binary_search::origin::Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        Solution::search_range_other_way(&nums, target)
    }

    fn find_index(nums: &Vec<i32>, target: i32, first: bool) -> i32 {
        // classic binary search
        let mut ans = -1;
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] < target {
                left = mid + 1;
            } else if nums[mid as usize] > target {
                right = mid - 1;
            } else {
                // 向左还是向右逼近
                if first {
                    right = mid - 1;
                    ans = mid;
                } else {
                    left = mid + 1;
                    ans = mid;
                }
            }
        }
        ans
    }

    fn search_range_other_way(nums: &Vec<i32>, target: i32) -> Vec<i32> {
        let left = nums.partition_point(|x| x < &target);
        if left == nums.len() || nums[left] != target {
            return vec![-1, -1];
        }

        let right = nums.partition_point(|x| x <= &target);
        return vec![left as i32, right as i32 - 1];
    }
}


#[test]
fn test_it() {
    assert_eq!(vec![3, 4], Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8));
    assert_eq!(vec![-1, -1], Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6));
    assert_eq!(vec![0, 0], Solution::search_range(vec![1], 1));
}