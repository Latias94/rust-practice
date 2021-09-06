struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut bytes = s.into_bytes();
        let mut left = 0;
        let mut right = 0;
        let len = bytes.len();
        while right < len {
            if right == len - 1 {
                Self::reverse_bytes(&mut bytes[left..right + 1]);
                break;
            } else if bytes[right] == b' ' {
                Self::reverse_bytes(&mut bytes[left..right]);
                left = right + 1;
            }
            right += 1;
        }
        unsafe { String::from_utf8_unchecked(bytes) }
        // or String::from_utf8(bytes).unwrap()
    }

    fn reverse_bytes(s: &mut [u8]) {
        if s.is_empty() {
            return;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            s.swap(left, right);
            left += 1;
            right -= 1;
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "Let's take LeetCode contest".to_string();
        let r = "s'teL ekat edoCteeL tsetnoc".to_string();
        assert_eq!(Solution::reverse_words(s), r);
    }
}
