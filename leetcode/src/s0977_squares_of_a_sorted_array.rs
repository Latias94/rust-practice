struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left = 0;
        let mut right = n - 1;
        let mut result = vec![0_i32; n];
        let mut k = right;
        while left <= right {
            if nums[left].abs() < nums[right].abs() {
                result[k] = nums[right].pow(2);
                right -= 1;
            } else {
                result[k] =nums[left].pow( 2);
                left += 1;
            }
            if k > 0 {
                k -= 1;
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
        let nums = vec![-4, -1, 0, 3, 10];
        let result = vec![0, 1, 9, 16, 100];
        assert_eq!(Solution::sorted_squares(nums), result);

        let nums = vec![-7, -3, 2, 3, 11];
        let result = vec![4, 9, 9, 49, 121];
        assert_eq!(Solution::sorted_squares(nums), result);
    }
}
