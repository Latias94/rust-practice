struct Solution;

//leetcode submit region begin(Prohibit modification and deletion)
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut cols: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        let mut boxes: Vec<Vec<bool>> = vec![vec![false; 9]; 9];
        for row in 0..9 {
            for col in 0..9 {
                let c = board[row][col];
                if c == '.' {
                    continue;
                }
                let idx = c as usize - '1' as usize;
                if rows[row][idx] {
                    return false;
                }
                if cols[col][idx] {
                    return false;
                }
                if boxes[(row / 3) * 3 + col / 3][idx] {
                    return false;
                }
                rows[row][idx] = true;
                cols[col][idx] = true;
                boxes[(row / 3) * 3 + col / 3][idx] = true;
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
