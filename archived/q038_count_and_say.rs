struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut cur: Vec<u8> = vec![1];
        let mut next = vec![];
        let mut m = 1;
        while m < n {
            let mut count = 1;
            for i in 0..cur.len() {
                if i < (cur.len() - 1) && cur[i + 1] == cur[i] {
                    count += 1;
                } else {
                    next.push(count);
                    next.push(cur[i]);
                    count = 1;
                }
            }
            cur = next;
            next = vec![];
            m += 1;
        }
        let ret = cur
            .into_iter()
            .map(|c| (c + '0' as u8) as char)
            .collect::<String>();
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q038() {
        assert_eq!("1".to_owned(), Solution::count_and_say(1));
        assert_eq!("11".to_owned(), Solution::count_and_say(2));
        assert_eq!("21".to_owned(), Solution::count_and_say(3));
        assert_eq!("1211".to_owned(), Solution::count_and_say(4));
        assert_eq!("111221".to_owned(), Solution::count_and_say(5));
        assert_eq!("312211".to_owned(), Solution::count_and_say(6));
    }
}
