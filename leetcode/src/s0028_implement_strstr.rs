struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let n = needle.len();
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        while fast <= haystack.len() {
            fast = slow + n;
            if fast > haystack.len() {
                break;
            }
            let mut same = true;
            for i in slow..fast {
                if haystack[i] != needle[i - slow] {
                    same = false;
                    break;
                }
            }
            if same {
                return slow as i32;
            } else {
                slow += 1;
            }
        }
        -1
    }

    #[allow(dead_code)]
    pub fn str_str_test(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        let n = needle.len();
        let mut slow: usize = 0;
        let mut fast: usize = 0;
        let haystack = haystack.into_bytes();
        let needle = needle.into_bytes();
        let first_c = needle[0];
        while fast <= haystack.len() {
            fast = slow + n;
            if fast > haystack.len() {
                break;
            }
            let mut temp = -1;
            let mut same = true;
            for i in slow..fast {
                if i > slow && temp == -1 && haystack[i] == first_c {
                    temp = i as i32;
                }
                if haystack[i] != needle[i - slow] {
                    same = false;
                    break;
                }
            }
            if !same {
                if temp != -1 {
                    slow = temp as usize;
                } else {
                    slow += 1;
                }
            } else {
                return slow as i32;
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
        assert_eq!(Solution::str_str("hello".to_string(), "ll".to_string()), 2);
        assert_eq!(
            Solution::str_str("mississippi".to_string(), "sippj".to_string()),
            -1
        );
        assert_eq!(
            Solution::str_str("aabaaabaaac".to_string(), "aabaaac".to_string()),
            4
        );
        assert_eq!(
            Solution::str_str("aaaaa".to_string(), "bba".to_string()),
            -1
        );
    }
}
