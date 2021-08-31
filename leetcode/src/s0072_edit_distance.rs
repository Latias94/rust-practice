struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let m = word1.len();
        let n = word2.len();
        let word1: Vec<char> = word1.chars().collect();
        let word2: Vec<char> = word2.chars().collect();
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for i in 0..=m {
            dp[i][0] = i;
        }
        for j in 0..=n {
            dp[0][j] = j;
        }
        for i in 1..=m {
            for j in 1..=n {
                if word1[i - 1] == word2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]).min(dp[i - 1][j - 1]) + 1;
                }
            }
        }

        dp[m][n] as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let word1 = "horse".to_string();
        let word2 = "ros".to_string();
        let res = 3;
        assert_eq!(Solution::min_distance(word1, word2), res);
        let word1 = "intention".to_string();
        let word2 = "execution".to_string();
        let res = 5;
        assert_eq!(Solution::min_distance(word1, word2), res);
    }
}
