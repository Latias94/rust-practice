struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                count = count.max(Self::dfs(&mut grid, i as i32, j as i32));
            }
        }
        count
    }

    pub fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let ui = i as usize;
        let uj = j as usize;
        if i < 0 || ui >= grid.len() || j < 0 || uj >= grid[0].len() || grid[ui][uj] == 0 {
            return 0;
        }
        grid[ui][uj] = 0;
        let mut count = 1;
        count += Self::dfs(grid, i - 1, j);
        count += Self::dfs(grid, i, j - 1);
        count += Self::dfs(grid, i + 1, j);
        count += Self::dfs(grid, i, j + 1);
        count
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid: Vec<Vec<i32>> = [
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0],
        ]
        .iter()
        .map(|v| v.to_vec())
        .collect();
        assert_eq!(Solution::max_area_of_island(grid), 6);
    }
}
