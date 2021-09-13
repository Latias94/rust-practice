struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let len = candidates.len();
        let mut result = Vec::new();
        let mut path = Vec::with_capacity(len);

        fn dfs(
            candidates: &[i32],
            len: usize,
            current_sum: i32,
            target: i32,
            last_index: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            use std::cmp::Ordering::*;
            match current_sum.cmp(&target) {
                Equal => result.push(path.to_vec()),
                Less => {
                    for i in last_index..len {
                        let num = candidates[i];
                        path.push(num);
                        dfs(candidates, len, current_sum + num, target, i, path, result);
                        path.pop(); // 回溯
                    }
                }
                Greater => {}
            }
        }
        dfs(&candidates, len, 0, target, 0, &mut path, &mut result);
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let candidates = vec![2, 3, 6, 7];
        let target = 7;
        let res = vec![vec![2, 2, 3], vec![7]];
        assert_eq!(Solution::combination_sum(candidates, target), res);
        let candidates = vec![2, 3, 5];
        let target = 8;
        let res = vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]];
        assert_eq!(Solution::combination_sum(candidates, target), res);
    }
}
