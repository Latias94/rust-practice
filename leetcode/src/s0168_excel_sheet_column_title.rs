struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn convert_to_title(column_number: i32) -> String {
        let mut result = Vec::new();
        let mut num = column_number;
        while num > 0 {
            num -= 1; // 0~25 => A~Z
            let digit = (num % 26) as u8;
            let char = (digit + b'A') as char;
            result.insert(0, char);
            num /= 26;
        }
        result.into_iter().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::convert_to_title(1), "A".to_string());
        assert_eq!(Solution::convert_to_title(28), "AB".to_string());
        assert_eq!(Solution::convert_to_title(701), "ZY".to_string());
        assert_eq!(
            Solution::convert_to_title(2147483647),
            "FXSHRXW".to_string()
        );
    }
}
