struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut pos1 = m - 1;
        let mut pos2 = n - 1;
        let mut cur = nums1.len() as i32 - 1;
        while pos1 >= 0 && pos2 >= 0 {
            let a = nums1[pos1 as usize];
            let b = nums2[pos2 as usize];
            if a < b {
                nums1[cur as usize] = b;
                pos2 -= 1;
            } else {
                nums1[cur as usize] = a;
                pos1 -= 1;
            }
            cur -= 1;
        }
        while cur >= 0 {
            while pos1 >= 0 {
                nums1[cur as usize] = nums1[pos1 as usize];
                cur -= 1;
                pos1 -= 1;
            }
            while pos2 >= 0 {
                nums1[cur as usize] = nums2[pos2 as usize];
                cur -= 1;
                pos2 -= 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q088() {
        let mut nums1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums2 = vec![2, 5, 6];
        Solution::merge(&mut nums1, 3, &mut nums2, 3);
        assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
    }
}
