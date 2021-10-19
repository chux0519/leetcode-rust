struct Solution;

use std::collections::HashMap;

impl Solution {
    /// 动态规划，先按照单词长度排序，然后遍历单词，利用一个 hash map 进行缓存
    /// 对于每个单词，遍历其缺失一个字母的所有可能的长度，取最大值 + 1 为其长度
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        let mut dp = HashMap::new();

        words.sort_unstable_by_key(|k| k.len());

        for word in words {
            let mut vs = vec![];
            for pos in 0..word.len() {
                let mut num = 0;
                let str = word.as_bytes();

                let sub_str = concat(&str[..pos], &str[pos + 1..]);
                let m = dp.get(&sub_str).unwrap_or(&0);
                num = *m + 1;
                vs.push(num);
            }
            dp.insert(word.clone(), *vs.iter().max().unwrap());
        }
        dbg!(dp.values());
        *dp.values().max().unwrap()
    }
}

fn toString(str: &[u8]) -> String {
    str.iter().map(|x| *x as char).collect()
}

fn concat(str1: &[u8], str2: &[u8]) -> String {
    let mut v = Vec::from(str1);
    v.extend_from_slice(str2);
    v.iter().map(|x| *x as char).collect()
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn q1048() {
        assert_eq!(
            4,
            Solution::longest_str_chain(vec![
                "a".to_owned(),
                "b".to_owned(),
                "ba".to_owned(),
                "bca".to_owned(),
                "bda".to_owned(),
                "bdca".to_owned()
            ])
        );
    }
}
