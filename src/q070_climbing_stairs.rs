struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // f(n) = f(n - 1) + f( n - 2 )
        let mut t = vec![0; n as usize + 1];
        t[0] = 1;
        t[1] = 1;
        for i in 2..=n as usize {
            t[i] = t[i - 1] + t[i - 2];
        }
        t[n as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q070() {
        assert_eq!(2, Solution::climb_stairs(2));
    }
}
