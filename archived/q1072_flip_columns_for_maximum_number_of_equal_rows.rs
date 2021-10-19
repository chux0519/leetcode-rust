struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let rows = matrix.len();

        let mut h = HashMap::<&Vec<i32>, i32>::new();
        for i in 0..rows {
            let row = &matrix[i];
            match h.get(row) {
                None => {
                    let mut res = 0;
                    for j in 0..rows {
                        if matrix[j] == matrix[i]
                            || matrix[j].iter().map(|n| 1 - n).collect::<Vec<i32>>() == matrix[i]
                        {
                            res += 1;
                        }
                    }
                    h.insert(row, res);
                }
                Some(_v) => continue,
            }
        }
        h.values().max().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1072() {
        assert_eq!(
            1,
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1],])
        );

        assert_eq!(
            2,
            Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0],])
        );

        assert_eq!(
            2,
            Solution::max_equal_rows_after_flips(
                vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0],]
            )
        );
    }
}
