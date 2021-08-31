struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut prev = 0;
        let mut max = i32::MIN;
        for num in nums {
            prev = num.max(prev + num);
            max = max.max(prev);
        }
        max
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maximum_subarray_1() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(Solution::max_sub_array(nums), 6);
        let nums = vec![-1];
        assert_eq!(Solution::max_sub_array(nums), -1);
        let nums = vec![1];
        assert_eq!(Solution::max_sub_array(nums), 1);
    }
}
