struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 || nums.len() == 1 {
            return nums.len() as i32;
        }
        let (mut i, mut j) = (0, 1);
        while i < nums.len() && j < nums.len() {
            if nums[i] != nums[j] {
                i += 1;
                nums[i] = nums[j];
            }
            j += 1;
        }
        (i + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q026() {
        assert_eq!(0, Solution::remove_duplicates(&mut vec![]));
        assert_eq!(2, Solution::remove_duplicates(&mut vec![1, 1, 2]));
        assert_eq!(
            5,
            Solution::remove_duplicates(&mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4])
        );
    }
}
