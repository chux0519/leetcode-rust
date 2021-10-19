struct Solution;

impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut arr = vec![a, b, c];
        arr.sort();
        if arr[2] - arr[0] == 2 {
            vec![0, 0]
        } else if arr[1] - arr[0] <= 2 || arr[2] - arr[1] <= 2 {
            // min: 1
            vec![1, arr[2] - arr[0] - 2]
        } else {
            vec![2, arr[2] - arr[0] - 2]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1033() {
        assert_eq!(vec![1, 2], Solution::num_moves_stones(1, 2, 5));
        assert_eq!(vec![0, 0], Solution::num_moves_stones(4, 3, 2));
        assert_eq!(vec![1, 2], Solution::num_moves_stones(3, 5, 1));
    }
}
