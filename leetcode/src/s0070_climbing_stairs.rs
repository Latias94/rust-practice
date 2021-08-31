struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }
        let mut one: i32 = 1;
        let mut two: i32 = 2;
        for _ in 3..=n {
            let temp: i32 = two;
            two += one;
            one = temp;
        }
        two
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::climb_stairs(1), 1);
        assert_eq!(Solution::climb_stairs(2), 2);
        assert_eq!(Solution::climb_stairs(3), 3);
        assert_eq!(Solution::climb_stairs(4), 5);
        assert_eq!(Solution::climb_stairs(5), 8);
        assert_eq!(Solution::climb_stairs(45), 1836311903);
    }
}
