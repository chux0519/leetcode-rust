/// DP
///
/// define dp[i]: how many soldiers on the left with rating smaller than rating[i]
/// do twice, reverse the line, and sum all the fit answers
struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let mut dp = vec![0; rating.len()];

        let mut ans = 0;
        for i in 0..rating.len() {
            for j in 0..i {
                if rating[j] < rating[i] {
                    dp[i] += 1;
                    ans += dp[j];
                }
            }
        }

        dp = vec![0; rating.len()];
        // reverse
        for i in 0..rating.len() {
            for j in 0..i {
                if rating[j] > rating[i] {
                    dp[i] += 1;
                    ans += dp[j];
                }
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1395() {
        assert_eq!(3, Solution::num_teams(vec![2, 5, 3, 4, 1]));
    }
}
