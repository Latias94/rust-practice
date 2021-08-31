struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let n = &nums.len();
        let k = k as usize % n;
        if k == 0 {
            return;
        }
        Self::rotate_array(nums, 0, n - 1);
        Self::rotate_array(nums, 0, k - 1);
        Self::rotate_array(nums, k, n - 1);
    }

    pub fn rotate_array(nums: &mut [i32], mut start: usize, mut end: usize) {
        while start < end {
            nums.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    #[allow(dead_code)]
    pub fn rotate_time_exceed(nums: &mut Vec<i32>, k: i32) {
        let n = nums.len();
        let k = k as usize % n;
        for _ in 0..k {
            for i in (1..n).rev() {
                nums.swap(i, i - 1);
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
        let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
        let k = 3;
        let result = vec![5, 6, 7, 1, 2, 3, 4];
        Solution::rotate(&mut nums, k);
        assert_eq!(nums, result);

        let mut nums = vec![-1, -100, 3, 99];
        let k = 2;
        let result = vec![3, 99, -1, -100];
        Solution::rotate(&mut nums, k);

        assert_eq!(nums, result);
        let mut nums = vec![1];
        Solution::rotate(&mut nums, 0);
        assert_eq!(nums, vec![1]);
    }
}
