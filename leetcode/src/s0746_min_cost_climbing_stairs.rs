struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::min;

impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let n = cost.len();
        let mut dp = vec![0; n];
        for i in 0..n {
            if i < 2 {
                dp[i] = cost[i];
            } else {
                dp[i] = min(dp[i - 1], dp[i - 2]) + cost[i];
            }
        }
        min(dp[n - 1], dp[n - 2])
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let cost1 = vec![10, 15, 20];
        assert_eq!(Solution::min_cost_climbing_stairs(cost1), 15);
        let cost2 = vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1];
        assert_eq!(Solution::min_cost_climbing_stairs(cost2), 6);
    }
}
