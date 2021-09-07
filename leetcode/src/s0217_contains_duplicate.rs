struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashSet::new();
        for num in nums {
            let success = map.insert(num);
            if !success {
                return true;
            }
        }
        false
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(Solution::contains_duplicate(nums), true);
        let nums = vec![1, 2, 3, 4];
        assert_eq!(Solution::contains_duplicate(nums), false);
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(Solution::contains_duplicate(nums), true);
    }
}
