struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut result = Vec::new();
        let mut used = vec![false; len];
        let mut path = Vec::with_capacity(len);

        fn dfs(
            nums: &[i32],
            len: usize,
            depth: i32,
            path: &mut Vec<i32>,
            used: &mut Vec<bool>,
            result: &mut Vec<Vec<i32>>,
        ) {
            if depth == len as i32 {
                result.push(path.to_vec());
                return;
            }
            for i in 0..len {
                if !used[i] {
                    path.push(nums[i]);
                    used[i] = true;
                    dfs(nums, len, depth + 1, path, used, result);
                    path.pop(); // 回溯
                    used[i] = false;
                }
            }
        }
        dfs(&nums, len, 0, &mut path, &mut used, &mut result);
        result
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3];
        let mut ans: Vec<Vec<i32>> = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ]
        .iter()
        .map(|v| v.to_vec())
        .collect();
        let mut res = Solution::permute(nums);
        ans.sort_unstable();
        res.sort_unstable();
        assert_eq!(res, ans);
    }
}
