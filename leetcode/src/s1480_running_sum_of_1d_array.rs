struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        result[0] = nums[0];
        for i in 1..n {
            result[i] = result[i - 1] + nums[i];
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
        let nums = vec![1, 2, 3, 4];
        let res = vec![1, 3, 6, 10];
        assert_eq!(Solution::running_sum(nums), res);
        let nums = vec![1, 1, 1, 1, 1];
        let res = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::running_sum(nums), res);
        let nums = vec![3, 1, 2, 10, 1];
        let res = vec![3, 4, 6, 16, 17];
        assert_eq!(Solution::running_sum(nums), res);
    }
}
