struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn delete_and_earn(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        for num in &nums {
            max = i32::max(max, *num);
        }
        let mut sum = vec![0; (max + 1) as usize];
        for num in &nums {
            sum[*num as usize] += num;
        }
        let mut dp = vec![0; (max + 1) as usize];
        let n = dp.len();
        dp[1] = sum[1];

        for i in 2..n {
            dp[i] = i32::max(dp[i - 1], dp[i - 2] + sum[i]);
        }
        dp[n - 1]
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![3, 4, 2];
        let res = 6;
        assert_eq!(Solution::delete_and_earn(nums), res);
        let nums = vec![2, 2, 3, 3, 3, 4];
        let res = 9;
        assert_eq!(Solution::delete_and_earn(nums), res);
    }
}
