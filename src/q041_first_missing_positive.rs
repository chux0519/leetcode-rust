struct Solution;

impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let nums = nums
            .into_iter()
            .filter(|x| *x <= n + 1 && *x > 0)
            .collect::<Vec<i32>>();
        if nums.is_empty() {
            return 1;
        }
        let max = nums.iter().max().unwrap();
        let min = nums.iter().min().unwrap();

        if *min > 1 {
            return 1;
        }
        let mut t = vec![0; (max - min) as usize + 2];
        for n in &nums {
            t[(*n - min + 1) as usize] = 1;
        }

        for i in 1..t.len() {
            if t[i] == 0 {
                return i as i32;
            }
        }
        max + 1
    }
}
#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q041() {
        assert_eq!(3, Solution::first_missing_positive(vec![1, 2, 0]));
        assert_eq!(2, Solution::first_missing_positive(vec![3, 4, 1, -1]));
        assert_eq!(1, Solution::first_missing_positive(vec![2]));
        assert_eq!(1, Solution::first_missing_positive(vec![2, 2]));
    }
}
