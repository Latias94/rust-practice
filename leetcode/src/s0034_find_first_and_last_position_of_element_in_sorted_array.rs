struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let left_bound = Self::get_bound(&nums, target, true);
        let right_bound = Self::get_bound(&nums, target, false);
        vec![left_bound, right_bound]
    }

    pub fn get_bound(nums: &[i32], target: i32, is_left: bool) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = -1;
        while left <= right {
            let mid = ((right - left) >> 1) + left;
            use std::cmp::Ordering::*;
            match target.cmp(&nums[mid as usize]) {
                Less => right = mid - 1,
                Greater => left = mid + 1,
                Equal => {
                    result = mid;
                    if is_left {
                        right = mid - 1;
                    } else {
                        left = mid + 1;
                    }
                }
            }
        }
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![5, 7, 7, 8, 8, 10];
        let target = 8;
        let res = vec![3, 4];
        assert_eq!(Solution::search_range(nums, target), res);
    }
}
