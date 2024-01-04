use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_set = HashSet::new();
        let mut column_set:Vec<HashSet<char>> = vec![HashSet::new(); 9];
        let mut grid_set:Vec<HashSet<char>> = vec![HashSet::new(); 9];
        for i in 0..9 {
            row_set.clear();
            for j in 0..9 {
                let num = &board[i][j];
                if *num == '.' { continue };
                // Check row
                if !row_set.insert(*num) {
                    return false;
                }
                // Check column
                if !column_set[j].insert(*num) {
                    return false;
                }
                // Check grid
                let grid_index = i/3*3 + j/3;
                if !grid_set[grid_index].insert(*num) {
                    return false;
                }
            }
        }
        return true;
    }
}

fn main() {
    let board = vec![vec!['5','3','.','.','7','.','.','.','.']
                    ,vec!['6','.','.','1','9','5','.','.','.']
                    ,vec!['.','9','8','.','.','.','.','6','.']
                    ,vec!['8','.','.','.','6','.','.','.','3']
                    ,vec!['4','.','.','8','.','3','.','.','1']
                    ,vec!['7','.','.','.','2','.','.','.','6']
                    ,vec!['.','6','.','.','.','.','2','8','.']
                    ,vec!['.','.','.','4','1','9','.','.','5']
                    ,vec!['.','.','.','.','8','.','.','7','9']];

    let result = Solution::is_valid_sudoku(board);
    println!("{:?}", result);
}