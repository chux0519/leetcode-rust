struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        // from right to left, save the carry
        let mut ret: Vec<u8> = vec![];
        let mut carry = 0;

        let bytes_a = a.as_bytes();
        let bytes_b = b.as_bytes();

        let mut bit_a = 0;
        let mut bit_b = 0;
        let mut sum = 0;
        let mut n = std::cmp::max(a.len(), b.len());
        for i in 1..=n {
            bit_a = if a.len() >= i {
                bytes_a[a.len() - i] as u8 - '0' as u8
            } else {
                0
            };
            bit_b = if b.len() >= i {
                bytes_b[b.len() - i] as u8 - '0' as u8
            } else {
                0
            };
            sum = bit_a + bit_b + carry;
            carry = sum / 2;
            sum %= 2;
            ret.push(sum + '0' as u8);
        }
        if carry > 0 {
            ret.push(carry + '0' as u8);
        }
        ret.reverse();
        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q067() {
        assert_eq!(
            "100".to_owned(),
            Solution::add_binary("11".to_owned(), "1".to_owned())
        );
        assert_eq!(
            "10101".to_owned(),
            Solution::add_binary("1010".to_owned(), "1011".to_owned())
        );
    }
}
