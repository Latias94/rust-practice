struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    // 记录【今天之前买入的最小值】
    // 计算【今天之前最小值买入，今天卖出的获利】，也即【今天卖出的最大获利】
    // 比较【每天的最大获利】，取最大值即可
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut max = 0;
        for i in 1..prices.len() {
            let price = prices[i];
            max = max.max(price - min);
            min = min.min(price);
        }
        max
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}
