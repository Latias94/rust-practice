struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let mut dp = vec![1; (row_index + 1) as usize];
        for i in 2..dp.len() {
            for j in (1..i).rev() {
                dp[j as usize] += dp[(j - 1) as usize]
            }
        }
        dp
    }

    // pub fn get_row(row_index: i32) -> Vec<i32> {
    //     Self::generate_pascals_triangle(row_index + 1).remove(row_index as usize)
    // }
    //
    // pub fn generate_pascals_triangle(num_rows: i32) -> Vec<Vec<i32>> {
    //     let mut dp = Vec::with_capacity(num_rows as usize);
    //     dp.push(vec![1; 1]);
    //     if num_rows == 1 {
    //         return dp;
    //     }
    //     dp.push(vec![1; 2]);
    //     for i in 2..num_rows {
    //         let mut layer = vec![1; (i + 1) as usize];
    //         for j in 1..i {
    //             layer[j as usize] =
    //                 dp[(i - 1) as usize][(j - 1) as usize] + dp[(i - 1) as usize][j as usize];
    //         }
    //         dp.push(layer);
    //     }
    //     dp
    // }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::get_row(4), vec![1, 4, 6, 4, 1]);
    }
}
