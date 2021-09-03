struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut left = 0_i32;
        let n = s.len();
        let mut right = (n - 1) as i32;
        let arr = s.as_bytes();
        while left < right {
            while left < n as i32 && !Self::is_num_or_alphabet(arr[left as usize]) {
                left += 1;
            }
            while right >= 0_i32 && !Self::is_num_or_alphabet(arr[right as usize]) {
                right -= 1;
            }
            if left >= n as i32 {
                return true;
            }
            if right < 0_i32 {
                return true;
            }
            if Self::to_upper(arr[left as usize]) != Self::to_upper(arr[right as usize]) {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }

    fn is_num_or_alphabet(c: u8) -> bool {
        (48..=57).contains(&c) || (65..=90).contains(&c) || (97..=122).contains(&c)
    }

    fn to_upper(c: u8) -> u8 {
        if (97..=122).contains(&c) {
            c - 32
        } else {
            c
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "A man, a plan, a canal: Panama".to_string();
        assert_eq!(Solution::is_palindrome(input), true);
        let input = "race a car".to_string();
        assert_eq!(Solution::is_palindrome(input), false);
        let input = ".,".to_string();
        assert_eq!(Solution::is_palindrome(input), true);
        let input = "0P".to_string();
        assert_eq!(Solution::is_palindrome(input), false);
    }
}
