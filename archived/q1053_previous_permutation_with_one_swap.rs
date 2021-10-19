struct Solution;

impl Solution {
    pub fn prev_perm_opt1(a: Vec<i32>) -> Vec<i32> {
        let mut ret = a.clone();
        let len = a.len();

        let mut pos = 0;
        for i in (1..len).rev() {
            if a[i] < a[i - 1] {
                pos = i;
                break;
            }
        }

        if pos != 0 {
            if pos == len - 1 {
                ret.swap(pos, pos - 1);
            } else {
                for x in (pos..len).rev() {
                    if a[x] < a[pos - 1] {
                        ret.swap(x, pos - 1);
                        break;
                    }
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
    fn q1053() {
        assert_eq!(vec![3, 1, 2], Solution::prev_perm_opt1(vec![3, 2, 1]));
        assert_eq!(vec![1, 1, 5], Solution::prev_perm_opt1(vec![1, 1, 5]));
        assert_eq!(
            vec![1, 7, 4, 6, 9],
            Solution::prev_perm_opt1(vec![1, 9, 4, 6, 7])
        );
        assert_eq!(vec![1, 1, 3, 3], Solution::prev_perm_opt1(vec![3, 1, 1, 3]));
    }
}
