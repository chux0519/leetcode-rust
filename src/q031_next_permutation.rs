struct Solution;

impl Solution {
    // 纯分析类题目，题解参考 https://leetcode.com/problems/next-permutation/solution/
    pub fn next_permutation(nums: &mut Vec<i32>) {
        let s = nums.as_mut_slice();
        let mut i = s.len() - 1;
        while i >= 1 && s[i - 1] >= s[i] {
            i -= 1;
        }
        if i >= 1 {
            let mut j = s.len() - 1;
            while s[j] <= s[i - 1] {
                j -= 1;
            }
            s.swap(i - 1, j);
        }
        s[i..].reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q031() {
        {
            let mut v = vec![1, 2, 3];
            Solution::next_permutation(v.as_mut());
            assert_eq!(vec![1, 3, 2], v);
        }
        {
            let mut v = vec![3, 2, 1];
            Solution::next_permutation(v.as_mut());
            assert_eq!(vec![1, 2, 3], v);
        }
        {
            let mut v = vec![1, 1, 5];
            Solution::next_permutation(v.as_mut());
            assert_eq!(vec![1, 5, 1], v);
        }
    }
}
