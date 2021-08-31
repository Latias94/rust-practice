struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;
        let mut meet_word = false;
        for c in s.chars().rev() {
            if c == ' ' {
                if meet_word {
                    break;
                }
            } else {
                meet_word = true;
                length += 1;
            }
        }
        length
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            Solution::length_of_last_word(" fly me to the moon ".to_string()),
            4
        );
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
    }
}
