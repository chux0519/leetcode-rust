struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    /// 直接用大顶堆
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap = BinaryHeap::new();
        for stone in stones {
            heap.push(stone);
        }
        while !heap.is_empty() {
            let first = heap.pop().unwrap();
            if heap.is_empty() {
                return first;
            }
            let second = heap.pop().unwrap();
            if first > second {
                heap.push(first - second);
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1046() {
        assert_eq!(1, Solution::last_stone_weight(vec![2, 7, 4, 1, 8, 1]));
    }
}
