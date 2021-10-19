struct Solution;

use std::collections::VecDeque;
impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut indegree = vec![0; num_courses as usize];
        // 寻找起点
        for pair in &prerequisites {
            // [1, 0] 的学习顺序是 0 -> 1，因此 pair[0] 的入度 + 1
            indegree[pair[0] as usize] += 1;
        }

        let mut q = VecDeque::new();
        for i in 0..indegree.len() {
            if indegree[i] == 0 {
                q.push_back(i as i32);
            }
        }

        let mut res = 0;
        while !q.is_empty() {
            let c = q.pop_front().unwrap();
            res += 1;
            for pair in &prerequisites {
                if pair[1] == c {
                    indegree[pair[0] as usize] -= 1;
                    if indegree[pair[0] as usize] == 0 {
                        q.push_back(pair[0]);
                    }
                }
            }
        }
        res == num_courses
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q207() {
        assert_eq!(true, Solution::can_finish(2, vec![vec![1, 0]]));
        assert_eq!(false, Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]));
    }
}
