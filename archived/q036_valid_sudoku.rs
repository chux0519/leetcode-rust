struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        // check row, check col, check sub boxes
        for row in &board {
            let mut t = Solution::init_table();
            for ch in row {
                if !Solution::check(ch, &mut t) {
                    return false;
                }
            }
        }

        for col in 0..9 {
            let mut t = Solution::init_table();
            for row in &board {
                if !Solution::check(&row[col], &mut t) {
                    return false;
                }
            }
        }

        for x in 0..3 {
            for y in 0..3 {
                let mut t = Solution::init_table();
                let left = x * 3;
                let right = x * 3 + 2;
                let top = y * 3;
                let bottom = y * 3 + 2;
                for i in left..=right {
                    for j in top..=bottom {
                        if !Solution::check(&board[i][j], &mut t) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn init_table() -> HashMap<char, bool> {
        let mut t = HashMap::new();
        for i in 1..=9 {
            t.insert((i + '0' as u8) as char, false);
        }
        t
    }
    fn check(ch: &char, h: &mut HashMap<char, bool>) -> bool {
        if *ch == '.' {
            return true;
        }
        if *h.get(ch).unwrap() == true {
            return false;
        } else {
            h.insert(*ch, true);
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q036() {
        assert_eq!(
            true,
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
        // row
        assert_eq!(
            false,
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['9', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
        // col
        assert_eq!(
            false,
            Solution::is_valid_sudoku(vec![
                vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['5', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
        // sub box
        assert_eq!(
            false,
            Solution::is_valid_sudoku(vec![
                vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
            ])
        );
    }
}
