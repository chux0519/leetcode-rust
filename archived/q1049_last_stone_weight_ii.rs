struct Solution;

impl Solution {
    /// https://www.geeksforgeeks.org/partition-a-set-into-two-subsets-such-that-the-difference-of-subset-sums-is-minimum/
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum_total = stones.iter().sum();
        // 暴力解法会超时 O(2^n)，这里想办法用 dp
        // Solution::last_stone_rec(&stones, stones.len(), 0, sum_total)

        // DP
        Solution::last_stone_dp(&stones, sum_total)
    }

    fn last_stone_rec(stones: &Vec<i32>, i: usize, sum_calculated: i32, sum_total: i32) -> i32 {
        use std::cmp;
        if i == 0 {
            return ((sum_total - sum_calculated) - sum_calculated).abs();
        }
        cmp::min(
            Solution::last_stone_rec(stones, i - 1, sum_calculated + stones[i - 1], sum_total),
            Solution::last_stone_rec(stones, i - 1, sum_calculated, sum_total),
        )
    }

    fn last_stone_dp(stones: &Vec<i32>, sum_total: i32) -> i32 {
        let n = stones.len();
        let mut dp = vec![vec![0; sum_total as usize + 1]; n + 1];

        // 每个 i，都可以选择除开这个 i，获得另一半的和为 0，因此 dp[i][0] = 1
        for i in 0..=n {
            dp[i][0] = 1;
        }

        // 0 个元素只可能满足和为 0，不可能和为其他
        for j in 1..=sum_total {
            dp[0][j as usize] = 0;
        }

        // 填表
        for i in 1..=n {
            for j in 1..=sum_total {
                // 除开 i
                dp[i][j as usize] = dp[i - 1][j as usize];

                // 包含 i
                if stones[i - 1] <= j {
                    dp[i][j as usize] |= dp[i - 1][(j - stones[i - 1]) as usize];
                }
            }
        }

        let mut diff = std::i32::MAX;
        // 选择离 sum / 2 最近的答案，越近，答案越小
        for j in (0..=(sum_total / 2)).rev() {
            if dp[n][j as usize] == 1 {
                diff = sum_total - 2 * j;
                break;
            }
        }
        diff
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q1049() {
        assert_eq!(1, Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]));
    }
}
