struct Solution;

impl Solution {
    /// 快慢指针，0ms, 2.4mb, 100%, 100%
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let (mut i, mut j) = (0, 0);
        while i < nums.len() && j < nums.len() {
            if nums[j] != val {
                nums[i] = nums[j];
                i += 1;
            }
            j += 1;
        }
        i as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q027() {
        assert_eq!(0, Solution::remove_element(&mut vec![], 0));
        assert_eq!(0, Solution::remove_element(&mut vec![1], 1));
        assert_eq!(2, Solution::remove_element(&mut vec![3, 2, 2, 3], 3));
        assert_eq!(
            5,
            Solution::remove_element(&mut vec![0, 1, 2, 2, 3, 0, 4, 2], 2)
        );
    }
}
