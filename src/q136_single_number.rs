struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut single = 0;
        for num in nums {
            single ^= num;
        }
        single
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q136() {
        assert_eq!(1, Solution::single_number(vec![2, 2, 1]));
        assert_eq!(4, Solution::single_number(vec![4, 1, 2, 1, 2]));
    }
}
