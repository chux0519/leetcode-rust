struct Solution;

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut sum = 0;
        let mut grumpy_pos = vec![];
        let len = grumpy.len();

        for i in 0..len {
            if grumpy[i] == 0 {
                sum += customers[i];
            } else {
                grumpy_pos.push(i);
            }
        }

        let mut sum_max = sum;
        // using a window, from left to right
        for p in grumpy_pos {
            let mut sum_x = sum + customers[p];

            for offset in 1..x as usize {
                sum_x += (if (p + offset < len) && grumpy[p + offset] == 1 {
                    customers[p + offset]
                } else {
                    0
                });
            }

            if sum_x > sum_max {
                sum_max = sum_x;
            }
        }
        sum_max
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q1052() {
        assert_eq!(
            16,
            Solution::max_satisfied(
                vec![1, 0, 1, 2, 1, 1, 7, 5],
                vec![0, 1, 0, 1, 0, 1, 0, 1],
                3
            )
        );
        assert_eq!(
            17,
            Solution::max_satisfied(vec![2, 6, 6, 9], vec![0, 0, 1, 1], 1)
        );
    }
}
