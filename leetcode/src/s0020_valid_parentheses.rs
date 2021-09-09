struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.into_bytes() {
            if c == b'(' || c == b'[' || c == b'{' {
                stack.push(c);
            } else {
                match stack.pop() {
                    Some(t) => {
                        if c == b')' && t != b'('
                            || c == b']' && t != b'['
                            || c == b'}' && t != b'{'
                        {
                            return false;
                        }
                    }
                    None => return false,
                }
            }
        }
        stack.is_empty()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::is_valid(String::from("()")), true);
        assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
        assert_eq!(Solution::is_valid(String::from("(]")), false);
        assert_eq!(Solution::is_valid(String::from("([)]")), false);
        assert_eq!(Solution::is_valid(String::from("{[]}")), true);
        assert_eq!(Solution::is_valid(String::from("]")), false);
    }
}
