struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn sum_odd_length_subarrays(arr: Vec<i32>) -> i32 {
        let n = arr.len();
        let mut prefix_sums = arr
            .iter()
            .scan(0, |sum, i| {
                *sum += i;
                Some(*sum)
            })
            .collect::<Vec<i32>>();
        prefix_sums.insert(0, 0);
        let mut sum = 0;
        for i in 0..n {
            for size in (1..prefix_sums.len()).step_by(2) {
                if i + size > n {
                    break;
                }
                sum += prefix_sums[i + size] - prefix_sums[i];
            }
        }
        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let arr = vec![1, 4, 2, 5, 3];
        let res = 58;
        assert_eq!(Solution::sum_odd_length_subarrays(arr), res);
        let arr = vec![1, 2];
        let res = 3;
        assert_eq!(Solution::sum_odd_length_subarrays(arr), res);
        let arr = vec![10, 11, 12];
        let res = 66;
        assert_eq!(Solution::sum_odd_length_subarrays(arr), res);
    }
}
