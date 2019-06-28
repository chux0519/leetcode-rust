struct Solution;

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        // use square and cycles( n / 2 )
        let n = matrix.len();
        let cycles = n / 2;

        let mut top_left = (0, 0);
        let mut top_right = (0, n - 1);
        let mut bottom_left = (n - 1, 0);
        let mut bottom_right = (n - 1, n - 1);

        let mut tmp = 0;

        for c in 0..cycles {
            top_left = (c, 0 + c);
            top_right = (c, n - 1 - c);
            bottom_left = (n - 1 - c, c);
            bottom_right = (n - 1 - c, n - 1 - c);
            dbg!(&top_right);
            for i in c..n - 1 - c {
                tmp = matrix[top_left.0][top_left.1];
                matrix[top_left.0][top_left.1] = matrix[bottom_left.0][bottom_left.1];
                matrix[bottom_left.0][bottom_left.1] = matrix[bottom_right.0][bottom_right.1];
                matrix[bottom_right.0][bottom_right.1] = matrix[top_right.0][top_right.1];
                matrix[top_right.0][top_right.1] = tmp;
                top_left.1 += 1; // right
                top_right.0 += 1; // down
                bottom_left.0 -= 1; // up
                bottom_right.1 -= 1; // left
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q048() {
        {
            let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
            let output = vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]];
            Solution::rotate(&mut input);
            assert_eq!(input, output);
        }
        {
            let mut input = vec![
                vec![5, 1, 9, 11],
                vec![2, 4, 8, 10],
                vec![13, 3, 6, 7],
                vec![15, 14, 12, 16],
            ];
            let output = vec![
                vec![15, 13, 2, 5],
                vec![14, 3, 4, 1],
                vec![12, 6, 8, 9],
                vec![16, 7, 10, 11],
            ];
            Solution::rotate(&mut input);
            assert_eq!(input, output);
        }
    }
}
