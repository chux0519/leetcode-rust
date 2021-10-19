struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        if prices.is_empty() {
            return 0;
        }

        for i in 1..prices.len() {
            profit = profit + std::cmp::max(prices[i] - prices[i - 1], 0);
        }
        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q122() {
        assert_eq!(7, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, Solution::max_profit(vec![1, 2, 3, 4, 5]));
    }
}
