use crate::binary_search::origin::Solution;

impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        // 有一个 i32 和 usize 的问题
        let (i, found) = match arr.binary_search(&x) {
            Ok(i) => (i as i32, true),
            Err(i) => (i as i32, false),
        };

        let n = arr.len();
        let mut left = if found { i } else { i - 1 };
        let mut right = left + 1;


        for _ in 0..k {
            if left < 0 {
                right += 1;
            } else if right as usize >= n || x - arr[left as usize] <= arr[right as usize] - x {
                left -= 1;
            } else {
                right += 1;
            }
        }

        arr[(left + 1) as usize..right as usize].to_vec()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_it() {
        assert_eq!(vec![1, 2, 2, 2], Solution::find_closest_elements(vec![1, 2, 2, 2, 5, 5, 5, 8, 9, 9], 4, 0));
        assert_eq!(vec![1, 2, 3, 4], Solution::find_closest_elements(vec![1, 2, 3, 4, 5], 4, 3));
    }
}





