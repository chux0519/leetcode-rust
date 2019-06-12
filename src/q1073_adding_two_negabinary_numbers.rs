struct Solution;

impl Solution {
    /// https://en.wikipedia.org/wiki/Negative_base#Addition
    /// 注意进位的规律：https://leetcode.com/problems/adding-two-negabinary-numbers/discuss/303795/C%2B%2B-From-Wikipedia
    /// If the sum for the current bit is greater than 2,
    /// the carry becomes -1, not 1 as in the base 2 case.
    /// Because of that, our sum can become -1. In this case, the carry should be set to 1.
    pub fn add_negabinary(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut ret = vec![];
        let n = if arr1.len() < arr2.len() {
            arr2.len()
        } else {
            arr1.len()
        };
        let mut carry = 0;

        let mut i = 1;
        while i <= n as i32 || carry != 0 {
            let idx1 = arr1.len() as i32 - i;
            let idx2 = arr2.len() as i32 - i;
            let n1 = if idx1 < 0 as i32 {
                0
            } else {
                arr1[idx1 as usize]
            };
            let n2 = if idx2 < 0 { 0 } else { arr2[idx2 as usize] };
            let sum = n1 + n2 + carry;
            carry = if sum >= 2 {
                -1
            } else {
                if sum == -1 {
                    1
                } else {
                    0
                }
            };
            ret.push(sum.abs() % 2);
            i += 1;
        }
        for i in (0..ret.len()).rev() {
            if ret[i] == 0 {
                if ret.len() > 1 {
                    ret.pop();
                }
            } else {
                break;
            }
        }
        ret.reverse();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1073() {
        assert_eq!(
            vec![1, 0, 0, 0, 0],
            Solution::add_negabinary(vec![1, 1, 1, 1, 1], vec![1, 0, 1])
        );
        assert_eq!(vec![0], Solution::add_negabinary(vec![0], vec![0]));
        assert_eq!(vec![0], Solution::add_negabinary(vec![1], vec![1, 1]));
        assert_eq!(vec![1, 1, 0], Solution::add_negabinary(vec![1], vec![1]));
    }
}
