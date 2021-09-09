struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut cols: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut boxes: Vec<HashSet<char>> = vec![HashSet::new(); 9];
        for row in 0..9 {
            for col in 0..9 {
                let c = board[row][col];
                if c == '.' {
                    continue;
                }
                if !rows[row].insert(c) {
                    return false;
                }
                if !cols[col].insert(c) {
                    return false;
                }
                if !boxes[(row / 3) * 3 + col / 3].insert(c) {
                    return false;
                }
            }
        }
        true
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;
    use rustgym_util::*;

    #[test]
    fn test() {
        let board = vec_vec_char![
            ['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        assert_eq!(Solution::is_valid_sudoku(board), true);
        let board = vec_vec_char![
            ['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            ['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            ['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            ['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            ['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            ['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            ['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            ['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            ['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ];
        assert_eq!(Solution::is_valid_sudoku(board), false);
    }
}
