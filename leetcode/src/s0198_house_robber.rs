struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering::*;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return nums[0];
        }
        let mut dp = vec![0; n];
        let compare = 2;
        for i in 0..n {
            match &i.cmp(&compare) {
                Less => dp[i] = nums[i],
                Equal => dp[2] = nums[0] + nums[2],
                Greater => dp[i] = i32::max(dp[i - 2], dp[i - 3]) + nums[i],
            }
        }
        i32::max(dp[n - 1], dp[n - 2])
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let inputs = vec![1, 2, 3, 1];
        assert_eq!(Solution::rob(inputs), 4);
        let inputs = vec![2, 7, 9, 3, 1];
        assert_eq!(Solution::rob(inputs), 12);
    }
}
