struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
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
        let mut input: Vec<char> = vec![];
        let output: Vec<char> = vec![];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);

        let mut input: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
        let output: Vec<char> = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);
    }
}
