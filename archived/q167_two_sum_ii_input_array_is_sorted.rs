struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut l = 0;
        let mut r = numbers.len() - 1;
        while l < r {
            let s = numbers[l] + numbers[r];
            if s == target {
                return vec![l as i32 + 1, r as i32 + 1];
            } else if s < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q167() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2]);
    }
}
