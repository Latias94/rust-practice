struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len() as usize;
        let n = grid[0].len() as usize;
        let mut dp = vec![vec![0; n]; m];
        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }
        for j in 1..n {
            dp[0][j] = dp[0][j - 1] + grid[0][j];
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = i32::min(dp[i - 1][j], dp[i][j - 1]) + grid[i][j];
            }
        }
        dp[m - 1][n - 1]
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;
    use rustgym_util::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::min_path_sum(vec_vec_i32![[1, 3, 1], [1, 5, 1], [4, 2, 1]]),
            7
        );
        assert_eq!(
            Solution::min_path_sum(vec_vec_i32![[1, 2, 3], [4, 5, 6]]),
            12
        );
    }
}
