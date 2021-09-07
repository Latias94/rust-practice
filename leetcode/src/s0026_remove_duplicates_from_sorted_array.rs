struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }
        let mut insert_index = 1;
        let mut cur_num = nums[0];
        for i in 1..n {
            if nums[i] != cur_num {
                nums[insert_index] = nums[i];
                cur_num = nums[i];
                insert_index += 1;
            }
        }
        insert_index as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut nums = vec![1, 1, 2];
        let expect = vec![1, 2];
        let count = 2;
        assert_eq!(Solution::remove_duplicates(&mut nums), count);
        assert_eq!(&nums[0..count as usize], &expect[..]);

        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let expect = vec![0, 1, 2, 3, 4];
        let count = 5;
        assert_eq!(Solution::remove_duplicates(&mut nums), count);
        assert_eq!(&nums[0..count as usize], &expect[..]);
    }
}
