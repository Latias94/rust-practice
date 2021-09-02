struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = false;
        let mut result = digits.clone();
        let n = digits.len();
        for i in (0..n).rev() {
            if i == n - 1 {
                result[i] += 1;
            }
            if carry {
                result[i] += 1;
            }
            carry = result[i] / 10 == 1;
            if !carry {
                break;
            }
            let remain = result[i] % 10;
            result[i] = remain;
        }
        if carry {
            result.insert(0, 1);
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
        let digits = vec![1, 2, 3];
        let result = vec![1, 2, 4];
        assert_eq!(Solution::plus_one(digits), result);

        let digits = vec![4, 3, 2, 1];
        let result = vec![4, 3, 2, 2];
        assert_eq!(Solution::plus_one(digits), result);

        let digits = vec![0];
        let result = vec![1];
        assert_eq!(Solution::plus_one(digits), result);

        let digits = vec![9, 9, 9];
        let result = vec![1, 0, 0, 0];
        assert_eq!(Solution::plus_one(digits), result);
    }
}
