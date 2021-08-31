struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering::*;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;
        while left <= right {
            let mid = (right - left) / 2 + left;
            match nums[mid].cmp(&target) {
                Equal => return mid as i32,
                Less => {
                    if mid + 1 > n - 1 {
                        break;
                    }
                    left = mid + 1;
                }
                Greater => {
                    if mid < 1 {
                        break;
                    }
                    right = mid - 1;
                }
            }
        }
        -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1 = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(Solution::search(nums1, 9), 4);

        let nums2 = vec![-1, 0, 3, 5, 9, 12];
        assert_eq!(Solution::search(nums2, 2), -1);

        assert_eq!(Solution::search(vec![5], -5), -1);

        assert_eq!(Solution::search(vec![2, 5], 2), 0);
    }
}
