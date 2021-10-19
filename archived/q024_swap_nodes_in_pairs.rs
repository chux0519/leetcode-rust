// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut inner_h) => match inner_h.next {
                None => Some(inner_h),
                Some(mut inner_second) => {
                    let third = inner_second.next;
                    inner_h.next = Solution::swap_pairs(third);
                    inner_second.next = Some(inner_h);
                    Some(inner_second)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};
    #[test]
    fn q024() {
        assert_eq!(None, Solution::swap_pairs(None));
        assert_eq!(
            Some(Box::new(ListNode::new(0))),
            Solution::swap_pairs(Some(Box::new(ListNode::new(0))))
        );
        assert_eq!(
            from_vec(vec![2, 1, 4, 3]),
            Solution::swap_pairs(from_vec(vec![1, 2, 3, 4]))
        );
    }

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        if v.is_empty() {
            return None;
        }
        let mut res = Some(Box::new(ListNode::new(0)));
        {
            let mut tmp = &mut res;
            for each in v {
                tmp = match { tmp }.as_mut() {
                    Some(n) => {
                        n.next = Some(Box::new(ListNode::new(each)));
                        &mut n.next
                    }
                    _ => unreachable!(),
                }
            }
        }

        res.unwrap().next
    }
}
