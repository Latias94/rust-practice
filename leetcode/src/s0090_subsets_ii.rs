struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(
            cur: usize,
            nums: &[i32],
            n: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            result.push(path.to_vec());
            if cur == n {
                return;
            }
            for i in cur..n {
                if i > cur && nums[i] == nums[i - 1] {
                    continue;
                }
                path.push(nums[i]);
                dfs(i + 1, nums, n, path, result);
                path.pop();
            }
        }
        nums.sort_unstable();
        let n = nums.len();
        let mut result = Vec::new();
        let mut path = Vec::new();
        dfs(0, &nums, n, &mut path, &mut result);
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
        let nums = vec![1, 2, 2];
        let mut res = vec_vec_i32![[2], [1], [1, 2, 2], [2, 2], [1, 2], []];
        res.sort_unstable();
        assert_eq!(Solution::subsets_with_dup(nums), res);
    }
}
