struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut result = 0;
        while num > 9 || num < -9 {
            result = result * 10 + num % 10;
            num /= 10;
        }
        let max = i32::MAX / 10;
        let min = i32::MIN / 10;

        num %= 10;
        if result > max || (result == max && num > 7) {
            return 0;
        }
        if result < min || (result == min && num < -8) {
            return 0;
        }
        result * 10 + num
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(10), 1);
        let x = 2_147_483_647;
        let res = 0;
        assert_eq!(Solution::reverse(x), res);
        let x = -2_147_483_412;
        let res = -2_143_847_412;
        assert_eq!(Solution::reverse(x), res);
        let x = 123_456_789;
        let res = 987_654_321;
        assert_eq!(Solution::reverse(x), res);
    }
}
