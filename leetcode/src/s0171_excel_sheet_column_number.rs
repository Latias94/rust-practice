struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut result = 0_i32;
        let title = column_title.bytes();

        for digit in title {
            result = result * 26 + (digit as i32 - 64) // 64 == b'A' - 1
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
        assert_eq!(Solution::title_to_number("A".to_string()), 1);
        assert_eq!(Solution::title_to_number("AB".to_string()), 28);
        assert_eq!(Solution::title_to_number("ZY".to_string()), 701);
        assert_eq!(Solution::title_to_number("FXSHRXW".to_string()), 2147483647);
    }
}
