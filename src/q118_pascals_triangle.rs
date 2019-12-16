struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        for level in 1..=num_rows as usize {
            let mut row: Vec<i32> = vec![0; level];
            // head
            row[0] = 1;
            // tail
            row[level - 1] = 1;
            if level > 2 {
                let last = &res[level - 2];
                for idx in 1..level - 1 {
                    row[idx] = last[idx] + last[idx - 1];
                }
            }
            res.push(row);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q118() {
        let output = vec![
            vec![1],
            vec![1, 1],
            vec![1, 2, 1],
            vec![1, 3, 3, 1],
            vec![1, 4, 6, 4, 1],
        ];
        assert_eq!(output, Solution::generate(5));
    }
}
