struct Solution;

impl Solution {
    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut nums = nums;
        nums.sort();
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
            // path will not be sorted in the loop
            // notice the condition below
            if i != start && path[i] == path[start] {
                continue;
            }
            path.swap(start, i);
            Solution::_permute(path.clone(), start + 1, result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q047() {
        assert_eq!(
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1],],
            Solution::permute_unique(vec![1, 1, 2])
        );
        assert_eq!(
            vec![
                vec![1, 1, 2, 2],
                vec![1, 2, 1, 2],
                vec![1, 2, 2, 1],
                vec![2, 1, 1, 2],
                vec![2, 1, 2, 1],
                vec![2, 2, 1, 1]
            ],
            Solution::permute_unique(vec![2, 2, 1, 1])
        );
    }
}
