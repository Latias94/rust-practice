struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort_unstable();
        let len = candidates.len();
        let mut result = Vec::new();
        let mut path = Vec::new();

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
                    // last_index 剪枝用
                    for i in last_index..len {
                        let num = candidates[i];
                        if i > last_index && num == candidates[i - 1] {
                            continue;
                        }
                        path.push(num);
                        dfs(candidates, len, current_sum + num, target, i+1, path, result);
                        path.pop(); // 回溯
                    }
                }
                Greater => {}
            }
        }
        dfs(&candidates, len, 0, target, 0, &mut path, &mut result);
        result.dedup();
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;
    use rustgym_util::*;

    #[test]
    fn test() {
        let candidates = vec![10, 1, 2, 7, 6, 1, 5];
        let target = 8;
        let mut res = vec_vec_i32![[1, 7], [1, 2, 5], [2, 6], [1, 1, 6]];
        res.sort_unstable();
        assert_eq!(Solution::combination_sum2(candidates, target), res);
    }
}
