struct Solution;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Solution::solve(board);
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                if board[x][y] == '.' {
                    for c in 1..=9 {
                        if Solution::is_valid(&board, x, y, (c + '0' as u8) as char) {
                            board[x][y] = (c + '0' as u8) as char;
                            if Solution::solve(board) {
                                return true;
                            } else {
                                board[x][y] = '.';
                            }
                        }
                    }
                    return false;
                }
            }
        }
        true
    }

    fn is_valid(board: &Vec<Vec<char>>, x: usize, y: usize, c: char) -> bool {
        for z in 0..9 {
            if board[x][z] == c
                || board[z][y] == c
                || board[3 * (x / 3) + z / 3][3 * (y / 3) + z % 3] == c
            {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q037() {
        let mut board = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let res = vec![
            vec!['5', '3', '4', '6', '7', '8', '9', '1', '2'],
            vec!['6', '7', '2', '1', '9', '5', '3', '4', '8'],
            vec!['1', '9', '8', '3', '4', '2', '5', '6', '7'],
            vec!['8', '5', '9', '7', '6', '1', '4', '2', '3'],
            vec!['4', '2', '6', '8', '5', '3', '7', '9', '1'],
            vec!['7', '1', '3', '9', '2', '4', '8', '5', '6'],
            vec!['9', '6', '1', '5', '3', '7', '2', '8', '4'],
            vec!['2', '8', '7', '4', '1', '9', '6', '3', '5'],
            vec!['3', '4', '5', '2', '8', '6', '1', '7', '9'],
        ];
        Solution::solve_sudoku(&mut board);
        assert_eq!(res, board);
    }
}
