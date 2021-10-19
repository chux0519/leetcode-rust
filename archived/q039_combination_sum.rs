struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut path = vec![];
        Solution::combine_recursive(&candidates, target, 0, &mut ret, &mut path);
        ret
    }

    fn combine_recursive(
        candidates: &Vec<i32>,
        target: i32,
        start: usize,
        ret: &mut Vec<Vec<i32>>,
        path: &mut Vec<i32>,
    ) {
        if target < 0 {
            return;
        }
        if target == 0 {
            ret.push(path.clone());
            return;
        }
        for i in start..candidates.len() {
            path.push(candidates[i]);
            Solution::combine_recursive(candidates, target - candidates[i], i, ret, path);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q039() {
        check(
            vec![vec![7], vec![2, 2, 3]],
            Solution::combination_sum(vec![2, 3, 6, 7], 7),
        );

        check(
            vec![vec![2, 2, 2, 2], vec![2, 3, 3], vec![3, 5]],
            Solution::combination_sum(vec![2, 3, 5], 8),
        );
    }

    fn check(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) {
        let mut a = a;
        let mut b = b;
        for each in &mut a {
            each.sort();
        }
        for each in &mut b {
            each.sort();
        }
        a.sort();
        b.sort();
        assert_eq!(a, b);
    }
}
