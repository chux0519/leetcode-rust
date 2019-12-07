struct Solution;
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        for i in (0..digits.len()).rev() {
            digits[i] += carry;
            if digits[i] >= 10 {
                digits[i] -= 10;
                carry = 1;
            } else {
                carry = 0;
                break;
            }
        }
        if carry > 0 {
            digits.insert(0, carry);
        }
        digits
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q066() {
        assert_eq!(vec![1], Solution::plus_one(vec![0]));
        assert_eq!(vec![1, 0], Solution::plus_one(vec![9]));
        assert_eq!(vec![1, 0, 0], Solution::plus_one(vec![9, 9]));
        assert_eq!(vec![1, 2, 4], Solution::plus_one(vec![1, 2, 3]));
        assert_eq!(vec![4, 3, 2, 2], Solution::plus_one(vec![4, 3, 2, 1]));
    }
}
