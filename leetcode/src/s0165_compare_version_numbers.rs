struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering;
impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let version1_num: Vec<u32> = version1
            .split('.')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        let version2_num: Vec<u32> = version2
            .split('.')
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        let len1 = version1_num.len();
        let len2 = version2_num.len();

        for i in 0..len1.max(len2) {
            match version1_num
                .get(i)
                .unwrap_or(&0)
                .cmp(version2_num.get(i).unwrap_or(&0))
            {
                Ordering::Greater => return 1,
                Ordering::Less => return -1,
                Ordering::Equal => {}
            }
        }
        0
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Solution::compare_version("1.01".to_string(), "1.001".to_string()),
            0
        );
        assert_eq!(
            Solution::compare_version("1.0".to_string(), "1.0.0".to_string()),
            0
        );
        assert_eq!(
            Solution::compare_version("0.1".to_string(), "1.1".to_string()),
            -1
        );
        assert_eq!(
            Solution::compare_version("1.0.1".to_string(), "1".to_string()),
            1
        );
        assert_eq!(
            Solution::compare_version("7.5.2.4".to_string(), "7.5.3".to_string()),
            -1
        );
    }
}
