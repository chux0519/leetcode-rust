struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        Solution::_permute(nums.clone(), 0, &mut ret);
        ret
    }

    fn _permute(path: Vec<i32>, start: usize, result: &mut Vec<Vec<i32>>) {
        let mut path = path;
        if start == path.len() {
            result.push(path);
            return;
        }
        for i in start..path.len() {
            path.swap(start, i);
            Solution::_permute(path.clone(), start + 1, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q046() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1]
            ],
            Solution::permute(vec![1, 2, 3])
        );
    }
}
