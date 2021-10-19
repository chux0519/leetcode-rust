struct Solution;

impl Solution {
    /// 思路是二分，每次确定比较量时，根据一定的规则进行修改
    /// 见 [Clever idea making it simple](https://leetcode.com/problems/search-in-rotated-sorted-array/discuss/14435/Clever-idea-making-it-simple)
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = (left + right) / 2;
            // If nums[mid] and target are "on the same side" of nums[0], we just take nums[mid]. Otherwise we use -infinity or +infinity as needed.
            let num = if (nums[0] <= target && nums[0] <= nums[mid])
                || (target < nums[0] && nums[mid] < nums[0])
            {
                nums[mid]
            } else {
                if target < nums[0] {
                    std::i32::MIN
                } else {
                    std::i32::MAX
                }
            };

            if target < num {
                right = mid;
            } else if target > num {
                left = mid + 1;
            } else {
                return mid as i32;
            }
        }

        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q033() {
        assert_eq!(4, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0));
        assert_eq!(-1, Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 3));
        assert_eq!(0, Solution::search(vec![1], 1));
    }
}
