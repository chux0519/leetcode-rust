struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum: i32 = nums.iter().sum();

        if sum % 2 != 0 {
            return false;
        }

        // target
        sum /= 2;

        let mut dp = vec![false; (sum + 1) as usize];
        dp[0] = true;

        for num in nums {
            for j in (0..=sum).rev() {
                if j >= num {
                    dp[j as usize] = dp[j as usize] || dp[(j - num) as usize];
                }
            }
        }

        dp[sum as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q0416() {
        assert_eq!(true, Solution::can_partition(vec![1, 5, 11, 5]));
        assert_eq!(false, Solution::can_partition(vec![1, 2, 3, 5]));
    }
}
