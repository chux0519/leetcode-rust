/// DP
///
/// matrix[i][j] = min(left, top, lefttop) + 1
/// sum all
struct Solution;

impl Solution {
    pub fn count_squares_(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let m = matrix.len();
        let n = matrix[0].len();
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 1 {
                    matrix[i][j] = std::cmp::min(
                        matrix[i][j - 1],
                        std::cmp::min(matrix[i - 1][j - 1], matrix[i - 1][j]),
                    ) + 1;
                }
            }
        }
        let mut ret = 0;
        for i in 0..m {
            for j in 0..n {
                // print!("{} ", matrix[i][j]);
                ret += matrix[i][j]
            }
            // println!("");
        }
        ret
    }

    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let mut matrix = matrix;
        let m = matrix.len();
        let n = matrix[0].len();
        let mut ret = 0;
        for i in 1..m {
            for j in 1..n {
                if matrix[i][j] == 1 {
                    matrix[i][j] = std::cmp::min(
                        matrix[i][j - 1],
                        std::cmp::min(matrix[i - 1][j - 1], matrix[i - 1][j]),
                    ) + 1;
                    ret += matrix[i][j];
                }
            }
        }
        for i in 0..m {
            ret += matrix[i][0];
        }
        for j in 1..n {
            ret += matrix[0][j]
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1277() {
        assert_eq!(1, Solution::count_squares(vec![vec![1]]));
        assert_eq!(
            7,
            Solution::count_squares(vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]])
        );
    }
}
