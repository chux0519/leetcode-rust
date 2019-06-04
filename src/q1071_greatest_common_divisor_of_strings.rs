struct Solution;

impl Solution {
    /// 辗转相除法的思想
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        if str1.len() == str2.len() {
            return if str1 == str2 { str1 } else { "".to_owned() };
        }
        if str1.len() < str2.len() {
            return Solution::gcd_of_strings(str2, str1);
        }
        if str2.len() == 0 {
            return str1;
        }
        if !str1.starts_with(&str2) {
            return "".to_owned();
        } else {
            return Solution::gcd_of_strings(str1.replacen(&str2, "", 1), str2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1071() {
        assert_eq!(
            "ABC".to_owned(),
            Solution::gcd_of_strings("ABCABC".to_owned(), "ABC".to_owned())
        )
    }
}
