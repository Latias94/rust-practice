struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::with_capacity(nums.len());
        for (index, num) in nums.iter().enumerate() {
            match map.get(&(target - num)) {
                None => {
                    map.insert(num, index);
                }
                Some(sub_index) => return vec![*sub_index as i32, index as i32],
            }
        }
        vec![]
    }
}
//leetcode submit region end(Prohibit modification and deletion)


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
    }
}