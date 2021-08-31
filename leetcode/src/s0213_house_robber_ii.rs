struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering::*;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        if n == 1 {
            return nums[0];
        }

        i32::max(
            Self::rob_slice(&nums[0..n - 1]),
            Self::rob_slice(&nums[1..n]),
        )
    }

    pub fn rob_slice(nums: &[i32]) -> i32 {
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
        let nums = vec![2, 3, 2];
        let res = 3;
        assert_eq!(Solution::rob(nums), res);
        let nums = vec![1, 2, 3, 1];
        let res = 4;
        assert_eq!(Solution::rob(nums), res);
        let nums = vec![0];
        let res = 0;
        assert_eq!(Solution::rob(nums), res);
        let nums = vec![];
        let res = 0;
        assert_eq!(Solution::rob(nums), res);
        let nums = vec![1];
        let res = 1;
        assert_eq!(Solution::rob(nums), res);
    }
}
