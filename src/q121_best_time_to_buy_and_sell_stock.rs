struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut profit = 0;
        if prices.is_empty() {
            return 0;
        }
        let mut cur = 0;
        for i in 1..prices.len() {
            cur = cur + prices[i] - prices[i - 1];
            cur = std::cmp::max(0, cur);
            profit = std::cmp::max(cur, profit);
        }

        profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q121() {
        assert_eq!(0, Solution::max_profit(vec![7, 6, 4, 3, 1]));
        assert_eq!(0, Solution::max_profit(vec![]));
        assert_eq!(1, Solution::max_profit(vec![1, 2]));
        assert_eq!(5, Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
    }
}
