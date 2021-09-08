struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut counter = vec![0; 26];
        let bytes = s.into_bytes();
        for b in &bytes {
            counter[(b - b'a') as usize] += 1;
        }
        for i in 0..bytes.len() {
            let b = bytes[i];
            if counter[(b - b'a') as usize] == 1 {
                return i as i32;
            }
        }
        -1
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()), 0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()), 2);
    }
}
