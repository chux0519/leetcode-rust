struct Solution;

impl Solution {
    /// 这里先采用暴力解法，发现可以直接通过，简单粗暴
    /// 官方还提供了两种思路，即使用一个 hashmap 进行缓存
    /// 进行两趟遍历，第一次用于初始化，第二次用于查找
    /// 在两趟遍历的基础上，可以优化成一趟遍历，这里也给出解法
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res = Vec::new();
        let len = nums.len();
        for i in 0..len {
            for j in i+1..len {
                if nums[i] + nums[j] == target {
                    res.push(i as i32);
                    res.push(j as i32);
                    return res;
                }
            }
        }
       res
    }

    pub fn two_pass_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut h = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            h.insert(n, i);
        }

        // second pass
        for (i, n) in nums.iter().enumerate() {
            if h.contains_key(&(target - n)) && *h.get(&(target - n)).unwrap() != i {
                return vec![i as i32, *h.get(&(target - n)).unwrap() as i32]
            }
        }
        vec![]
   }
    pub fn one_pass_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut h = HashMap::new();

        for (i, n) in nums.iter().enumerate() {
            match h.get(&(target - n)) {
                None => { h.insert(n, i); },
                Some(idx) => { return vec![*idx as i32, i as i32]; }
            }
        }
        vec![]
   }


}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn run() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![0, 1], Solution::two_pass_hashmap(vec![2, 7, 11, 15], 9));
        assert_eq!(vec![0, 1], Solution::one_pass_hashmap(vec![2, 7, 11, 15], 9));
    }
}
