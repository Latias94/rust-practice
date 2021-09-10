struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        while fast < nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }
        slow as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![3, 2, 2, 3];
        let val = 3;
        assert_eq!(Solution::remove_element(&mut nums, val), 2);
        assert_eq!(nums[..2], vec![2, 2]);
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let val = 2;
        assert_eq!(Solution::remove_element(&mut nums, val), 5);
        assert_eq!(nums[..5], vec![0, 1, 3, 0, 4]);
    }
}
