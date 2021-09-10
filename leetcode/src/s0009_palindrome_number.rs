struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        match x {
            num if num < 0 => false,
            _ => x == Self::reverse(x),
        }
    }

    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut result = 0;
        while num != 0 {
            result = result * 10 + num % 10;
            num /= 10;
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
        let x = -123;
        let res = false;
        assert_eq!(Solution::is_palindrome(x), res);
        let x = 12321;
        let res = true;
        assert_eq!(Solution::is_palindrome(x), res);

        assert_eq!(Solution::reverse(123), 321);
        assert_eq!(Solution::reverse(-123), -321);
        assert_eq!(Solution::reverse(120), 21);
        assert_eq!(Solution::reverse(0), 0);
        assert_eq!(Solution::reverse(10), 1);
        let x = 123_456_789;
        let res = 987_654_321;
        assert_eq!(Solution::reverse(x), res);
    }
}
