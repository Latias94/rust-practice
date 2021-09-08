struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        if nums1.len() > nums2.len() {
            return Self::intersect(nums2, nums1);
        }
        let mut intersection = Vec::new();
        let mut map = nums1.into_iter().fold(HashMap::new(), |mut map, num| {
            *map.entry(num).or_insert(0) += 1;
            map
        });

        for num in nums2 {
            let count = map.entry(num).or_insert(0);
            if *count > 0 {
                *count -= 1;
                intersection.push(num);
            }
        }
        intersection
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
        assert_eq!(Solution::intersect(nums1, nums2), vec![2, 2]);
        let nums1 = vec![4, 9, 5];
        let nums2 = vec![9, 4, 9, 8, 4];
        assert_eq!(Solution::intersect(nums1, nums2), vec![9, 4]);
        let nums1 = vec![1, 2];
        let nums2 = vec![1, 1];
        assert_eq!(Solution::intersect(nums1, nums2), vec![1]);
    }
}
