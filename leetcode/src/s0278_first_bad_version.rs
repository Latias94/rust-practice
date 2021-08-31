struct Solution {
    bad: i32,
}

impl Solution {
    fn new(bad: i32) -> Self {
        Solution { bad }
    }

    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.bad
    }
}

//leetcode submit region begin(Prohibit modification and deletion)
// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut left = 1;
        let mut right = n;
        while left < right {
            let mid = (right - left) / 2 + left;
            if !self.isBadVersion(mid) {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bad = 4;
        let solution = Solution::new(bad);
        let n = 5;
        let res = 4;
        assert_eq!(solution.first_bad_version(n), res);

        let bad = 1;
        let solution = Solution::new(bad);
        let n = 1;
        let res = 1;
        assert_eq!(solution.first_bad_version(n), res);
    }
}
