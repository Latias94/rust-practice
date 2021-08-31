pub struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::cmp::Ordering::*;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut low = 0;
        let mut high = nums.len();
        while low < high {
            let mid = (high - low) / 2 + low;
            match nums[mid].cmp(&target) {
                Equal => return mid as i32,
                Less => low = mid + 1,
                Greater => high = mid,
            }
        }
        low as i32
    }
}
//leetcode submit region end(Prohibit modification and deletion)

// for testing, see: benches/leetcode.rs
impl Solution {
    pub fn search_insert_first(nums: Vec<i32>, target: i32) -> i32 {
        for (i, num) in nums.iter().enumerate() {
            if num >= &target {
                return i as i32;
            }
        }
        nums.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 3, 5, 6];
        let target = 5;
        assert_eq!(Solution::search_insert(nums, target), 2);
        let nums = vec![1, 3, 5, 6];
        let target = 2;
        assert_eq!(Solution::search_insert(nums, target), 1);
        let nums = vec![1, 3, 5, 6];
        let target = 7;
        assert_eq!(Solution::search_insert(nums, target), 4);
        let nums = vec![1, 3, 5, 6];
        let target = 0;
        assert_eq!(Solution::search_insert(nums, target), 0);
    }
}
