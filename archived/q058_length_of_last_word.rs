struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res = 0;
        let s_bytes = s.as_bytes();
        for i in (0..s_bytes.len()).rev() {
            if s_bytes[i] == b' ' {
                if res == 0 {
                    continue;
                }
                return res;
            }
            res += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q058() {
        assert_eq!(5, Solution::length_of_last_word("hello, world".to_owned()));
    }
}
