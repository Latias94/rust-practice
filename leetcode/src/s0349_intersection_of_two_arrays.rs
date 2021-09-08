struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set = nums1.into_iter().collect::<HashSet<i32>>();
        let mut intersection = Vec::new();
        for num in nums2 {
            if set.contains(&num) {
                intersection.push(num);
            }
        }
        intersection.into_iter().collect::<HashSet<i32>>().into_iter().collect()
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums1 = vec![1, 2, 2, 1];
        let nums2 = vec![2, 2];
        assert_eq!(Solution::intersection(nums1, nums2), vec![2]);
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(Solution::intersection(nums1, nums2), vec![9, 4]);
    }
}
