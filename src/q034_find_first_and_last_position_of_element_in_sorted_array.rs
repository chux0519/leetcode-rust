struct Solution;

impl Solution {
    /// 暴力解法，顺利 AC
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut ret = vec![-1, -1];
        for (idx, n) in nums.iter().enumerate() {
            if *n == target {
                if ret[0] == -1 {
                    ret[0] = idx as i32;
                }
                ret[1] = idx as i32;
            }
            if *n > target {
                break;
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q034() {
        assert_eq!(
            vec![3, 4],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 8)
        );
        assert_eq!(
            vec![-1, -1],
            Solution::search_range(vec![5, 7, 7, 8, 8, 10], 6)
        );
    }
}
