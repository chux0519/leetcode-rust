struct Solution;

impl Solution {
    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        return Solution::combine_recursive(&candidates, target, 0);
    }

    fn combine_recursive(candidates: &Vec<i32>, target: i32, start: usize) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        for i in start..candidates.len() {
            let cand = candidates[i];
            let mut tmp = vec![];
            let mut t = target;
            while t - cand >= 0 {
                tmp.push(cand);
                let mut solutions = Solution::combine_recursive(&candidates, t - cand, i + 1);
                if !solutions.is_empty() {
                    for s in &mut solutions {
                        s.extend(tmp.clone());
                    }
                }
                ret.extend(solutions);
                t -= cand;
                if t == 0 {
                    ret.push(tmp.clone());
                    break;
                }
            }
        }
        ret
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
