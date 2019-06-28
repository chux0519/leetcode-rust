struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        // fn(nums, i) = fn(nums, i -1) > 0 ? fn(nums, i-1) : 0 + nums[i];
        let mut dp = vec![0; nums.len() + 1];
        dp[0] = 0;

        for i in 1..=nums.len() {
            dp[i] = if dp[i - 1] > 0 { dp[i - 1] } else { 0 } + nums[i - 1];
        }

        let mut iter = dp.iter();
        iter.next(); // skip index 0
        *iter.max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q053() {
        assert_eq!(
            6,
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
        );
    }
}
