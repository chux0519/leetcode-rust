struct Solution;

impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        // do it like do multiply manualy
        let mut res: Vec<i32> = vec![0; num1.len() + num2.len()];

        let mut pos_1 = 0;
        let mut pos_2 = 0;

        let bytes_1 = num1.as_bytes();
        let bytes_2 = num2.as_bytes();
        for i in (0..num1.len()).rev() {
            let n1 = bytes_1[i] as u8 - '0' as u8;
            let mut carry: i32 = 0;
            pos_2 = 0;
            for j in (0..num2.len()).rev() {
                let n2 = bytes_2[j] as u8 - '0' as u8;
                let mut result = (n1 * n2) as i32 + res[pos_1 + pos_2] + carry;
                carry = result / 10;
                result %= 10;
                res[pos_1 + pos_2] = result;
                pos_2 += 1;
            }
            if carry > 0 {
                res[pos_1 + pos_2] += carry;
            }
            pos_1 += 1;
        }
        let mut ret: Vec<u8> = vec![];

        let mut start = res.len() - 1;
        while res[start] == 0 {
            // num1 or num2 is 0
            if start == 0 {
                return "0".to_owned();
            }
            start -= 1;
        }

        for i in (0..=start).rev() {
            ret.push(res[i] as u8 + '0' as u8);
        }
        String::from_utf8(ret).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q048() {
        assert_eq!(
            "56088".to_owned(),
            Solution::multiply("123".to_owned(), "456".to_owned())
        );
        assert_eq!(
            "0".to_owned(),
            Solution::multiply("0".to_owned(), "456".to_owned())
        );
    }
}
