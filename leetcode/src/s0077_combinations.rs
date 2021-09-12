struct Solution;

use rustgym_util::*;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        let mut path = Vec::with_capacity(n as usize - 1);

        fn dfs(n: i32, k: i32, begin: i32, path: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
            if path.len() == k as usize {
                result.push(path.to_vec());
                return;
            }
            // 剪枝
            for i in begin..=n - (k - path.len() as i32) + 1 {
                path.push(i);
                dfs(n, k, i + 1, path, result);
                path.pop();
            }
        }
        dfs(n, k, 1, &mut path, &mut result);
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 4;
        let k = 2;
        let mut res = vec_vec_i32![[2, 4], [3, 4], [2, 3], [1, 2], [1, 3], [1, 4]];
        let mut ans = Solution::combine(n, k);
        res.sort();
        ans.sort();
        assert_eq!(ans, res);
    }
}
