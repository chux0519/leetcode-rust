struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        if needle.is_empty() {
            return 0;
        }
        match &haystack.find(&needle) {
            None => -1,
            Some(pos) => *pos as i32,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q028() {
        assert_eq!(0, Solution::str_str("hello".into(), "".into()));
        assert_eq!(2, Solution::str_str("hello".into(), "ll".into()));
        assert_eq!(-1, Solution::str_str("aaaaa".into(), "bba".into()));
    }
}
