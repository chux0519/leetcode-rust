struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let bytes = s.as_bytes();
        let mut head = 0;
        let mut tail = bytes.len() as i32 - 1;
        loop {
            if head > tail {
                break;
            }
            if !bytes[head as usize].is_ascii_alphanumeric() {
                head += 1;
            } else if !bytes[tail as usize].is_ascii_alphanumeric() {
                tail -= 1;
            } else {
                if bytes[head as usize] == bytes[tail as usize]
                    || bytes[head as usize].to_ascii_lowercase()
                        == bytes[tail as usize].to_ascii_lowercase()
                {
                    head += 1;
                    tail -= 1;
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q125() {
        assert_eq!(
            true,
            Solution::is_palindrome("A man, a plan, a canal: Panama".to_owned())
        );
        assert_eq!(false, Solution::is_palindrome("race a car".to_owned()));
        assert_eq!(true, Solution::is_palindrome(" ".to_owned()));
        assert_eq!(true, Solution::is_palindrome("a.".to_owned()));
        assert_eq!(false, Solution::is_palindrome("0P".to_owned()));
    }
}
