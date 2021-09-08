struct Solution;

static mut X: i32 = 0;
unsafe fn guess(num: i32) -> i32 {
    use std::cmp::Ordering::*;
    match X.cmp(&num) {
        Equal => 0,
        Greater => 1,
        Less => -1,
    }
}
//leetcode submit region begin(Prohibit modification and deletion)
/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is lower than the guess number
 *			      1 if num is higher than the guess number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    #[allow(non_snake_case)]
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left <= right {
            let mid = left + ((right - left) >> 1);
            match guess(mid) {
                -1 => right = mid,
                1 => left = mid + 1,
                0 => return mid,
                _ => {}
            }
        }
        1
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        unsafe {
            // X = 6;
            // assert_eq!(Solution::guessNumber(10), 6);
            X = 2;
            assert_eq!(Solution::guessNumber(2), 2);
        }
    }
}
