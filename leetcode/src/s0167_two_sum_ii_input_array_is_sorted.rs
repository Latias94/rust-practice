struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering;
impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l < r {
            let sum = numbers[l] + numbers[r];
            match sum.cmp(&target) {
                Ordering::Equal => break,
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            }
        }
        vec![(l + 1) as i32, (r + 1) as i32]
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
        assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3]);
        assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2]);
    }
}
