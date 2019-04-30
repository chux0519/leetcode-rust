// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution;

impl Solution {
    /// 这里使用递归解法，初衷是考虑代码会比较少一点，但似乎还是比较长
    /// 这里使用 dummy node 使得 head 可以 mut

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        // 使用 dummy 节点，使得 head 可以 mutable
        let mut dummy = Box::new(ListNode{val: 0, next: head});
        let pre = &mut dummy;
        let mut start = pre.next.take();


        let mut count = 0;
        let mut cur = &mut start;

        while count < k {
            match cur {
                None => {
                    break;
                },
                Some(inner_node) => {
                    cur = &mut inner_node.next;
                }
            };
            count += 1;
        }
        if count == k {
            // 这里 cur 是第 k+1 个元素，即下一组需要被反转的头节点
            // 递归
            let mut next_group = Solution::reverse_k_group(cur.take(), k);
            cur = &mut next_group;

            // 这里先声明，避免生命周期不够长 (overwritten before being read)
            let mut prev = None::<Box<ListNode>>;

            while count > 0 {
                // 从 cur 开始头部插入
                match start {
                    Some(mut inner_start) => {
                        let next = inner_start.next;
                        inner_start.next = cur.take();
                        start = next;
                        prev = Some(inner_start);
                        cur = &mut prev;
                    },
                    _ => unreachable!()
                };
                count -= 1;
            }
            start = cur.take();
        }
        start
    }
}

#[cfg(test)]
mod tests {
    use super::{ListNode, Solution};
    #[test]
    fn q025() {
        assert_eq!(None, Solution::reverse_k_group(None, 1));
        assert_eq!(
            from_vec(vec![2, 1, 4, 3, 5]),
            Solution::reverse_k_group(from_vec(vec![1, 2, 3, 4, 5]), 2)
        );
        assert_eq!(
            from_vec(vec![3, 2, 1, 4, 5]),
            Solution::reverse_k_group(from_vec(vec![1, 2, 3, 4, 5]), 3)
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
