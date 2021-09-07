struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn balanced_string_split(s: String) -> i32 {
        let bytes = s.into_bytes();
        let mut result = 0;
        let mut count = 0;
        for c in bytes {
            count += if c == b'L' { 1 } else { -1 };
            if count == 0 {
                result += 1;
            }
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
        let s = "RLRRLLRLRL".to_string();
        assert_eq!(Solution::balanced_string_split(s), 4);
        let s = "RLLLLRRRLR".to_string();
        assert_eq!(Solution::balanced_string_split(s), 3);
        let s = "LLLLRRRR".to_string();
        assert_eq!(Solution::balanced_string_split(s), 1);
        let s = "RLRRRLLRLL".to_string();
        assert_eq!(Solution::balanced_string_split(s), 2);
    }
}
