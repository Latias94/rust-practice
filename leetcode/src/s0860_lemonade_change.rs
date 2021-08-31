struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn lemonade_change(bills: Vec<i32>) -> bool {
        let mut b5 = 0;
        let mut b10 = 0;

        for bill in bills {
            match (bill, b5 > 0, b10 > 0, b5 > 2) {
                (5, _, _, _) => b5 += 1,
                (10, true, _, _) => {
                    b10 += 1;
                    b5 -= 1;
                }
                (20, true, true, _) => {
                    b10 -= 1;
                    b5 -= 1;
                }
                (20, _, _, true) => {
                    b5 -= 3;
                }
                _ => return false,
            }
        }
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let bills = vec![5, 5, 5, 10, 20];
        assert_eq!(Solution::lemonade_change(bills), true);
        let bills = vec![5, 5, 10];
        assert_eq!(Solution::lemonade_change(bills), true);
        let bills = vec![10, 10];
        assert_eq!(Solution::lemonade_change(bills), false);
        let bills = vec![5, 5, 10, 10, 20];
        assert_eq!(Solution::lemonade_change(bills), false);
        let bills = vec![
            5, 5, 10, 20, 5, 5, 5, 5, 5, 5, 5, 5, 5, 10, 5, 5, 20, 5, 20, 5,
        ];
        assert_eq!(Solution::lemonade_change(bills), true);
        let bills = vec![5, 5, 5, 5, 20, 20, 5, 5, 5, 5];
        assert_eq!(Solution::lemonade_change(bills), false);
    }
}
