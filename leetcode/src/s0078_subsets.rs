struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn dfs(
            cur: usize,
            nums: &[i32],
            n: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            use std::cmp::Ordering::*;
            match cur.cmp(&n) {
                Equal => result.push(path.to_vec()),
                Less => {
                    path.push(nums[cur]);
                    dfs(cur + 1, nums, n, path, result);
                    path.pop();
                    dfs(cur + 1, nums, n, path, result);
                }
                Greater => {}
            }
        }
        let n = nums.len();
        let mut result = Vec::new();
        let mut path = Vec::new();
        dfs(0, &nums, n, &mut path, &mut result);
        result
    }

    // 修改组合的代码，改变长度
    #[allow(dead_code)]
    pub fn subsets_from_combination(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut result = Vec::new();
        let mut path = Vec::with_capacity(len);

        fn dfs(
            nums: &[i32],
            len: usize,
            current_sum: i32,
            target_len: usize,
            last_index: usize,
            path: &mut Vec<i32>,
            result: &mut Vec<Vec<i32>>,
        ) {
            use std::cmp::Ordering::*;
            match path.len().cmp(&target_len) {
                Equal => result.push(path.to_vec()),
                Less => {
                    for i in last_index..len {
                        let num = nums[i];
                        path.push(num);
                        dfs(
                            nums,
                            len,
                            current_sum + num,
                            target_len,
                            i + 1,
                            path,
                            result,
                        );
                        path.pop();
                    }
                }
                Greater => {}
            }
        }
        for i in 0..=len {
            dfs(&nums, len, 0, i, 0, &mut path, &mut result);
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
        let nums = vec![0];
        let res = vec![vec![0], vec![]];
        assert_eq!(Solution::subsets(nums), res);
    }
}
