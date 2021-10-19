use std::cmp::Ordering;

struct Solution;

impl Solution {
    /// 直接二分即可
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32;
        while left < right {
            let mid = (left + right) / 2;
            match nums[mid as usize].cmp(&target) {
                Ordering::Equal => return mid,
                Ordering::Greater => {
                    right = mid;
                }
                Ordering::Less => {
                    left = mid + 1;
                }
            }
        }

        left
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q035() {
        assert_eq!(2, Solution::search_insert(vec![1, 3, 5, 6], 5));
        assert_eq!(1, Solution::search_insert(vec![1, 3, 5, 6], 2));
        assert_eq!(4, Solution::search_insert(vec![1, 3, 5, 6], 7));
        assert_eq!(0, Solution::search_insert(vec![1, 3, 5, 6], 0));
    }
}
