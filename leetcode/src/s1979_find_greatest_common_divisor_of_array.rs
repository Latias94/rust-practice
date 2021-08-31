struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn find_gcd(nums: Vec<i32>) -> i32 {
        let mut min = nums[0];
        let mut max = nums[0];
        for num in &nums[1..] {
            if num > &max {
                max = *num;
            } else if num < &min {
                min = *num;
            }
        }
        Self::gcd(min, max)
    }

    fn gcd(mut n: i32, mut m: i32) -> i32 {
        assert!(n != 0 && m != 0);
        while m != 0 {
            if m < n {
                std::mem::swap(&mut m, &mut n);
            }
            m %= n;
        }
        n
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_gcd(vec![2, 5, 6, 9, 10]), 2);
        assert_eq!(Solution::find_gcd(vec![7, 5, 6, 8, 3]), 1);
        assert_eq!(Solution::find_gcd(vec![3, 3]), 3);
    }
}
