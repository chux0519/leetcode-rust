struct Solution;

impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut count = 0;
        for n in nums {
            if count == 0 {
                res = n;
                count += 1;
            } else if n == res {
                count += 1;
            } else {
                count -= 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q169() {
        assert_eq!(5, Solution::majority_element(vec![6, 5, 5]));
    }
}
