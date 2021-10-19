struct Solution;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        // left[i], right[i] stores highest to the left / right;
        // then compute water
        let n = height.len();
        let mut left = vec![0; n];
        let mut right = vec![0; n];

        if n == 0 {
            return 0;
        }

        left[0] = height[0];
        for i in 1..n {
            left[i] = std::cmp::max(left[i - 1], height[i]);
        }

        right[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right[i] = std::cmp::max(right[i + 1], height[i]);
        }

        let mut water = 0;
        for i in 0..n {
            water += (std::cmp::min(left[i], right[i]) - height[i]);
        }
        water
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q042() {
        assert_eq!(6, Solution::trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]));
        assert_eq!(0, Solution::trap(vec![]));
    }
}
