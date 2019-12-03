struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        // 寻找 O 是否存在到 border 的路径
        // 如果存在，那么它和在路径上的点都不会被翻转
        // 如果不存在，那么它和它相邻的 O 都会被翻转
        let m = board.len();
        if m == 0 {
            return;
        }
        let n = board[0].len();
        if n == 0 {
            return;
        }

        // find 2 rows and 2 lines
        for j in 0..n {
            let i = 0;
            if board[i][j] == 'O' {
                Solution::bfs_x_y(board, i, j);
            }
            let i = m - 1;
            if board[i][j] == 'O' {
                Solution::bfs_x_y(board, i, j);
            }
        }
        for i in 0..m {
            let j = 0;
            if board[i][j] == 'O' {
                Solution::bfs_x_y(board, i, j);
            }
            let j = n - 1;
            if board[i][j] == 'O' {
                Solution::bfs_x_y(board, i, j);
            }
        }

        for i in 0..m {
            for j in 0..n {
                match board[i][j] {
                    'O' => {
                        board[i][j] = 'X';
                    }
                    'B' => {
                        board[i][j] = 'O';
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn bfs_x_y(board: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let mut q: VecDeque<(i32, i32)> = VecDeque::new();
        let m = board.len() as i32;
        let n = board[0].len() as i32;
        q.push_back((i as i32, j as i32));
        while !q.is_empty() {
            let (_i, _j) = q.pop_front().unwrap();
            if _i >= 0 && _i < m && _j >= 0 && _j < n && board[_i as usize][_j as usize] == 'O' {
                board[_i as usize][_j as usize] = 'B';
                q.push_back((_i - 1, _j));
                q.push_back((_i + 1, _j));
                q.push_back((_i, _j - 1));
                q.push_back((_i, _j + 1));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q130() {
        {
            let mut input = vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ];
            let output = vec![
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            ];
            Solution::solve(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = vec![vec!['O']];
            let output = vec![vec!['O']];
            Solution::solve(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = [
                ['X', 'O', 'O', 'X', 'X', 'X', 'O', 'X', 'O', 'O'],
                ['X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'X', 'X'],
                ['X', 'O', 'X', 'X', 'X', 'O', 'X', 'X', 'X', 'O'],
                ['O', 'X', 'X', 'X', 'O', 'X', 'O', 'X', 'O', 'X'],
                ['X', 'X', 'O', 'X', 'X', 'O', 'O', 'X', 'X', 'X'],
                ['O', 'X', 'X', 'O', 'O', 'X', 'O', 'X', 'X', 'O'],
                ['O', 'X', 'X', 'X', 'X', 'X', 'O', 'X', 'X', 'X'],
                ['X', 'O', 'O', 'X', 'X', 'O', 'X', 'X', 'O', 'O'],
                ['X', 'X', 'X', 'O', 'O', 'X', 'O', 'X', 'X', 'O'],
            ]
            .iter()
            .cloned()
            .map(|sub| sub.iter().cloned().collect())
            .collect();
            let output: Vec<Vec<char>> = [
                ['X', 'O', 'O', 'X', 'X', 'X', 'O', 'X', 'O', 'O'],
                ['X', 'O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O'],
                ['O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                ['O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O'],
                ['O', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'X'],
                ['X', 'X', 'X', 'X', 'X', 'X', 'X', 'X', 'O', 'O'],
                ['X', 'X', 'X', 'O', 'O', 'X', 'O', 'X', 'X', 'O'],
            ]
            .iter()
            .cloned()
            .map(|sub| sub.iter().cloned().collect())
            .collect();

            Solution::solve(&mut input);
            assert_eq!(input, output);
        }
    }
}
