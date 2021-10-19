struct Solution;

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut m_heights = heights.clone();
        m_heights.sort();
        let mut n = 0;
        for i in 0..heights.len() {
            if m_heights[i] != heights[i] {
                n += 1;
            }
        }
        n
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1051() {
        assert_eq!(3, Solution::height_checker(vec![1, 1, 4, 2, 1, 3]));
    }
}
