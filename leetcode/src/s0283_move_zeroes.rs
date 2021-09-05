struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut j = 0;
        for i in 0..nums.len() {
            let num = nums[i];
            if num != 0 {
                if i == j {
                    j += 1;
                    continue;
                }
                nums[i] = 0;
                nums[j] = num;
                j += 1;
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);

        let mut nums = vec![2, 1];
        Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![2, 1]);
    }
}
