/// DP bottom-up
///
/// | n | a | e | i | o | i |
/// | --- | --- | --- | --- | --- | --- |
/// | 1 | 5 | 4 | 3 | 2 | 1 |
/// | 2 | 15 | 10 | 6 | 3 | 1 |
///
/// dp[n][j] = dp[n-1][j] + dp[n][j+1]
struct Solution;

impl Solution {
    pub fn _count_vowel_strings(n: i32) -> i32 {
        let mut dp = Vec::new();
        // n = 0
        dp.push(vec![0, 0, 0, 0, 0]);
        // n = 1
        dp.push(vec![5, 4, 3, 2, 1]);

        for i in 2..=n as usize {
            dp.push(vec![0, 0, 0, 0, 0]);
            for j in (0..=4).rev() {
                dp[i][j] = dp[i - 1][j] + if j + 1 <= 4 { dp[i][j + 1] } else { 0 };
            }
        }

        return dp[n as usize][0];
    }

    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut a = 5;
        let mut e = 4;
        let mut i = 3;
        let mut o = 2;
        let mut u = 1;
        for _ in 1..n {
            u = u;
            o += u;
            i += o;
            e += i;
            a += e;
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1641() {
        assert_eq!(5, Solution::count_vowel_strings(1));
        assert_eq!(66045, Solution::count_vowel_strings(33));
    }
}
