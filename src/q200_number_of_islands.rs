struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut islands = 0;
        let m = grid.len();
        if m == 0 {
            return 0;
        }
        let n = grid[0].len();
        if n == 0 {
            return 0;
        }

        let mut grid = grid;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == '1' {
                    Solution::bfs_x_y(&mut grid, i, j);
                    islands += 1;
                }
            }
        }
        islands
    }

    pub fn bfs_x_y(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let m = grid.len();
        if m == 0 {
            return;
        }
        let n = grid[0].len();
        if n == 0 {
            return;
        }
        let mut q = VecDeque::new();
        q.push_back((i as i32, j as i32));
        while !q.is_empty() {
            let (_i, _j) = q.pop_front().unwrap();
            if _i >= 0
                && _i < m as i32
                && _j >= 0
                && _j < n as i32
                && grid[_i as usize][_j as usize] == '1'
            {
                q.push_back((_i - 1, _j));
                q.push_back((_i + 1, _j));
                q.push_back((_i, _j - 1));
                q.push_back((_i, _j + 1));
                grid[_i as usize][_j as usize] = '0';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q200() {
        {
            let input = vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', 'O', '1', '0'],
                vec!['1', '1', 'O', '0', '0'],
                vec!['0', 'O', '0', '0', '0'],
            ];
            assert_eq!(Solution::num_islands(input), 1);
        }
        {
            let input = vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', 'O', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', 'O', '0', '1', '1'],
            ];
            assert_eq!(Solution::num_islands(input), 3);
        }
    }
}
