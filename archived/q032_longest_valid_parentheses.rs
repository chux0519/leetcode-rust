struct Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::new();
        let mut max = 0;

        stack.push(-1);
        for (i, c) in s.chars().enumerate() {
            if c == '(' {
                stack.push(i as i32);
            } else {
                stack.pop();
                if stack.is_empty() {
                    stack.push(i as i32);
                } else {
                    let len = i as i32 - stack[stack.len() - 1];
                    max = if max > len { max } else { len };
                }
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q032() {
        assert_eq!(2, Solution::longest_valid_parentheses("(()".into()));
        assert_eq!(4, Solution::longest_valid_parentheses(")()())".into()));
        assert_eq!(6, Solution::longest_valid_parentheses("()(())".into()));
        assert_eq!(2, Solution::longest_valid_parentheses("()(()".into()));
    }
}
