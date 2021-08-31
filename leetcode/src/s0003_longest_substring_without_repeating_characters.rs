struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map: HashMap<char, usize> = HashMap::new();
        let mut max: usize = 0;
        let mut left: usize = 0;
        for (right, c) in s.char_indices() {
            if let Some(end) = map.insert(c, right) {
                left = usize::max(left, end + 1);
            }
            max = usize::max(right - left + 1, max);
        }
        max as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = " ".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
        let s = "abba".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 2);
        let s = "abcabcbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
        let s = "bbbb".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 1);
        let s = "pwwkew".to_string();
        assert_eq!(Solution::length_of_longest_substring(s), 3);
    }
}
