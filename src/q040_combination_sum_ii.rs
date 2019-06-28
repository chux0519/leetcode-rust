struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut path = vec![];
        let mut candidates = candidates;
        candidates.sort();
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
            if i > start && candidates[i] == candidates[i - 1] {
                continue;
            }
            path.push(candidates[i]);
            Solution::combine_recursive(candidates, target - candidates[i], i + 1, ret, path);
            path.pop();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q040() {
        check(
            vec![vec![1, 7], vec![1, 2, 5], vec![2, 6], vec![1, 1, 6]],
            Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8),
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
