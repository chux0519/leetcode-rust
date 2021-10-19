struct Solution;

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let root = vec![1];
        let level_1 = vec![1, 1];
        if row_index == 0 {
            return root;
        }
        if row_index == 1 {
            return level_1;
        }
        let mut row = vec![1; row_index as usize + 1];
        let mut level = 2;
        loop {
            let mut last = 1;
            let mid = level as usize / 2;
            for i in 1..=mid {
                row[i] = row[i] + last;
                last = row[i] - last;
            }
            for i in (mid + 1)..level as usize {
                row[i] = row[level as usize - i];
            }
            level += 1;
            if level > row_index {
                break;
            }
        }
        row
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q119() {
        assert_eq!(vec![1], Solution::get_row(0));
        assert_eq!(vec![1, 1], Solution::get_row(1));
        assert_eq!(vec![1, 2, 1], Solution::get_row(2));
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
        assert_eq!(vec![1, 4, 6, 4, 1], Solution::get_row(4));
        assert_eq!(vec![1, 5, 10, 10, 5, 1], Solution::get_row(5));
    }
}
