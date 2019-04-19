struct Solution;

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
impl Solution {
    /// 最容易想到的是两两合并
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
      let mut ret = None::<Box<ListNode>>;
      for entry in lists {
        ret = merge_two_lists(ret, entry);
      }
      ret
    }
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2 :Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let mut h1 = l1;
  let mut h2 = l2;
  let mut res = Some(Box::new(ListNode::new(0)));
  {
    // 限制 tmp 的作用域
    let mut tmp = &mut res;
    loop {
      let node1 = h1.take();
      let node2 = h2.take();
      tmp = match {tmp}.as_mut() {
        Some(_n) => {
            if node1.is_some() && node2.is_some() {
            let inner1 = node1.unwrap();
            let inner2 = node2.unwrap();
            let v1 = inner1.val;
            let v2 = inner2.val;
            if v1 < v2 {
              _n.next = Some(Box::new(ListNode::new(v1)));
              h1 = inner1.next;
              h2 = Some(inner2);
            } else {
              _n.next = Some(Box::new(ListNode::new(v2)));
              h2 = inner2.next;
              h1 = Some(inner1);
            }
          } else {
            if node1.is_none() && node2.is_some() {
              _n.next = node2;
            } else if node2.is_none() && node1.is_some() {
              _n.next = node1;
            }
            break;
          }
          &mut _n.next
        }
        _ => unreachable!()
      };
    }
  }
  res.unwrap().next
}


#[cfg(test)]
mod tests {
    use super::{ListNode, Solution, merge_two_lists};
    #[test]
    fn q023() {
        assert_eq!(
        from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]), 
        Solution::merge_k_lists(vec![
          from_vec(vec![1, 4, 5]),
          from_vec(vec![1, 3, 4]),
          from_vec(vec![2, 6]),
        ]));
    }
    
    #[test]
    fn q023_merge_two() {
      assert_eq!(None, merge_two_lists(None, None));
      assert_eq!(Some(Box::new(ListNode::new(1))), merge_two_lists(None, Some(Box::new(ListNode::new(1)))));
      assert_eq!(from_vec(vec![1,1,2,3,4,5]), merge_two_lists(from_vec(vec![1,4,5]), from_vec(vec![1,2,3])));
    }

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
      if v.is_empty() {
        return None;
      }
      let mut res = Some(Box::new(ListNode::new(0)));
      {
        let mut tmp = &mut res;
        for each in v {
          tmp = match {tmp}.as_mut() {
            Some(n)=> {
              n.next = Some(Box::new(ListNode::new(each)));
              &mut n.next
            }
            _ => unreachable!()
          }
        }
      }
      
      res.unwrap().next
    }
}
