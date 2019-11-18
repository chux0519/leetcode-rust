struct Solution;

use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn find_ladders(
        begin_word: String,
        end_word: String,
        word_list: Vec<String>,
    ) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        let mut paths: VecDeque<Vec<String>> = VecDeque::new();
        paths.push_back(vec![begin_word.clone()]);

        let mut cur_level = 1;
        let mut min_level = 65535;

        let mut visited: HashSet<String> = HashSet::new();

        let mut dict: HashSet<String> = HashSet::new();
        for word in &word_list {
            dict.insert(word.clone());
        }

        while !paths.is_empty() {
            // treat path as the BFS element
            let path = paths.pop_front().unwrap();
            let mut _dict = dict.clone();
            for s in &path {
                _dict.remove(s);
            }

            if path.len() > cur_level {
                if path.len() > min_level {
                    continue;
                }
                cur_level += 1;
                for v in &visited {
                    _dict.remove(v);
                }
                visited.clear();
            }

            'letter: for idx in 0..end_word.len() {
                let mut last_word = Vec::from(path[path.len() - 1].as_bytes());
                if onediff(&last_word, end_word.as_bytes()) && _dict.contains(&end_word) {
                    // find one path
                    let mut new_path = path.clone();
                    if min_level == 65535 {
                        min_level = cur_level;
                    }
                    new_path.push(end_word.clone());
                    ret.push(new_path);
                    break 'letter;
                }
                for c in 0..26 {
                    last_word[idx] = 'a' as u8 + c;
                    let s = String::from_utf8(last_word.clone()).unwrap();
                    if _dict.contains(&s) {
                        let mut new_path = path.clone();
                        _dict.remove(&s);
                        new_path.push(s.clone());
                        paths.push_back(new_path);
                        visited.insert(s);
                    }
                }
            }
        }

        ret
    }
}

fn onediff(from: &[u8], to: &[u8]) -> bool {
    let mut diffs = 0;
    for x in 0..from.len() {
        if from[x] != to[x] {
            diffs += 1;
        }
    }
    if diffs == 1 {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::Solution;
    #[test]
    fn q126() {
        {
            let begin_word = "hit".to_owned();
            let end_word = "cog".to_owned();
            let word_list = vec![
                "hot".to_owned(),
                "dot".to_owned(),
                "dog".to_owned(),
                "lot".to_owned(),
                "log".to_owned(),
                "cog".to_owned(),
            ];

            assert_eq!(
                vec![
                    vec![
                        "hit".to_owned(),
                        "hot".to_owned(),
                        "dot".to_owned(),
                        "dog".to_owned(),
                        "cog".to_owned()
                    ],
                    vec![
                        "hit".to_owned(),
                        "hot".to_owned(),
                        "lot".to_owned(),
                        "log".to_owned(),
                        "cog".to_owned()
                    ]
                ],
                Solution::find_ladders(begin_word, end_word, word_list)
            );
        }
        {
            let begin_word = "red".to_owned();
            let end_word = "tax".to_owned();
            let word_list = vec![
                "ted".to_owned(),
                "tex".to_owned(),
                "red".to_owned(),
                "tax".to_owned(),
                "tad".to_owned(),
                "den".to_owned(),
                "rex".to_owned(),
                "pee".to_owned(),
            ];

            assert_eq!(
                vec![
                    vec![
                        "red".to_owned(),
                        "ted".to_owned(),
                        "tad".to_owned(),
                        "tax".to_owned()
                    ],
                    vec![
                        "red".to_owned(),
                        "ted".to_owned(),
                        "tex".to_owned(),
                        "tax".to_owned()
                    ],
                    vec![
                        "red".to_owned(),
                        "rex".to_owned(),
                        "tex".to_owned(),
                        "tax".to_owned()
                    ]
                ],
                Solution::find_ladders(begin_word, end_word, word_list)
            );
        }
    }
}
