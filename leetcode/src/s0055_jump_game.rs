struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let mut max_distance = 0_usize;
        for i in 0..n {
            if i > max_distance {
                return false;
            }
            let distance = i + nums[i] as usize;
            max_distance = usize::max(max_distance, distance);
        }
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
        assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
        assert_eq!(Solution::can_jump(vec![0]), true);
        assert_eq!(Solution::can_jump(vec![0, 2, 3]), false);
        assert_eq!(Solution::can_jump(vec![1, 0, 1, 0]), false);
    }
}
