struct Solution;

impl Solution {
    /// 使用加减法即可，同时将 divisor 进行位移运算以加速计算
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 1 {
            return dividend;
        }
        if dividend == i32::min_value() && divisor == -1 {
            return i32::max_value();
        }
        let minus = (dividend < 0 && divisor > 0) || (dividend > 0 && divisor < 0);
        let (mut dvd, dvs) = (i64::abs(dividend as i64), i64::abs(divisor as i64));
        let mut ret = 0;
        let mut shift = 0;
        let mut sub = dvs;
        while dvd >= dvs {
            if dvd >= sub {
                dvd -= sub;
                ret += 1 << shift;
                sub <<= 1;
                shift += 1;
            } else {
                sub >>= 1;
                shift -= 1;
            }
        }
        if minus {
            -ret
        } else {
            ret
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q029() {
        assert_eq!(1, Solution::divide(1, 1));
        assert_eq!(3, Solution::divide(10, 3));
        assert_eq!(-2, Solution::divide(7, -3));
        assert_eq!(2147483647, Solution::divide(-2147483648, -1));
        assert_eq!(-2147483648, Solution::divide(-2147483648, 1));
    }
}
