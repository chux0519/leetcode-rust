struct Solution;

impl Solution {
    /// 直接使用栈
    pub fn remove_duplicates(s: String) -> String {
        let mut stack = Vec::new();
        for c in s.as_bytes() {
            if !stack.is_empty() && stack[stack.len() - 1] == *c {
                stack.pop();
            } else {
                stack.push(*c);
            }
        }
        stack.into_iter().map(|c| c as char).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1047() {
        assert_eq!(
            "ca".to_owned(),
            Solution::remove_duplicates("abbaca".to_owned())
        );
    }
}
