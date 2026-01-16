use std::collections::HashSet;

pub struct Solution;
impl Solution {
    pub fn _36_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut check = true;
        for i in 0..9 {
            let mut row_set = HashSet::new();
            let row_check = board[i].iter().all(|x| {
                if (*x == '.') {
                    return true;
                }
                row_set.insert(x)
            });

            // Rule 2: Col check
            let mut col_set: HashSet<&char> = HashSet::new();
            let col_check = (0..9).all(|x| {
                if (board[x][i] == '.') {
                    return true;
                }
                col_set.insert(&board[x][i])
            });
            // Rule 1: Block check
            let mut block_set: HashSet<&char> = HashSet::new();
            let row_start = (i / 3) * 3;
            let col_start = (i % 3) * 3;
            let block: Vec<char> = board[row_start..row_start + 3] // Slice the 3 relevant rows
                .iter()
                .flat_map(|row| &row[col_start..col_start + 3]) // Slice the 3 relevant columns
                .copied()
                .collect();
            let block_check = block.iter().all(|x| {
                if *x == '.' {
                    return true;
                }
                block_set.insert(x)
            });
            if (row_check && col_check && block_check) == false {
                check = false;
                break;
            }
        }
        check
    }
    pub fn run() {
        let board: Vec<Vec<char>> = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '8', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let result = &self::Solution::_36_valid_sudoku(board);
        println!("{:?}", result);
    }
}
