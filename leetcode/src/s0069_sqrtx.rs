struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut result: i32 = -1;
        let mut left: usize = 0;
        let x = x as usize;
        let mut right = x;
        while left <= right {
            let mid = ((right - left) >> 1) + left;
            use std::cmp::Ordering::*;
            match x.cmp(&(mid * mid)) {
                Less => right = mid - 1,
                Greater => {
                    result = mid as i32;
                    left = mid + 1;
                },
                Equal => {
                    result = mid as i32;
                    break;
                }
            };
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
        assert_eq!(Solution::my_sqrt(4), 2);
        assert_eq!(Solution::my_sqrt(8), 2);
    }
}
