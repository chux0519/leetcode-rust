/// using heap start
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Node(i32, usize);

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        // 小顶堆，因此这里进行 reverse 操作
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.0 == other.0
    }
}

impl Eq for Node {}

/// using heap end

struct Solution;

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
impl Solution {
    /// 最容易想到的是两两合并 (564ms, 3.4mb)
    /// 其次是使用小顶堆 (4ms, 4.9mb)
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut ret = None::<Box<ListNode>>;
        for entry in lists {
            ret = merge_two_lists(ret, entry);
        }
        ret
    }

    pub fn merge_using_heap(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Node> = BinaryHeap::new();
        // 基本的思想就是，遍历所有链表的所有节点，全部加入到小顶堆
        // 然后利用堆特性进行重建链表
        for (index, entry) in lists.iter().enumerate() {
            entry
                .as_ref()
                .and_then(|node| Some(heap.push(Node(node.val, index))));
        }
        Solution::next(lists, &mut heap)
    }

    pub fn next(
        mut lists: Vec<Option<Box<ListNode>>>,
        heap: &mut BinaryHeap<Node>,
    ) -> Option<Box<ListNode>> {
        heap.pop().map(|node| {
            let next = lists[node.1].take().unwrap().next;
            next.as_ref()
                .and_then(|n| Some(heap.push(Node(n.val, node.1))));
            lists[node.1] = next;
            Box::new(ListNode {
                val: node.0,
                next: Solution::next(lists, heap),
            })
        })
    }
}

fn merge_two_lists(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut h1 = l1;
    let mut h2 = l2;
    let mut res = Some(Box::new(ListNode::new(0)));
    {
        // 限制 tmp 的作用域
        let mut tmp = &mut res;
        loop {
            let node1 = h1.take();
            let node2 = h2.take();
            tmp = match { tmp }.as_mut() {
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
                _ => unreachable!(),
            };
        }
    }
    res.unwrap().next
}

#[cfg(test)]
mod tests {
    use super::{merge_two_lists, ListNode, Solution};
    #[test]
    fn q023() {
        assert_eq!(
            from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]),
            Solution::merge_k_lists(vec![
                from_vec(vec![1, 4, 5]),
                from_vec(vec![1, 3, 4]),
                from_vec(vec![2, 6]),
            ])
        );
        assert_eq!(
            from_vec(vec![1, 1, 2, 3, 4, 4, 5, 6]),
            Solution::merge_using_heap(vec![
                from_vec(vec![1, 4, 5]),
                from_vec(vec![1, 3, 4]),
                from_vec(vec![2, 6]),
            ])
        );
    }

    #[test]
    fn q023_merge_two() {
        assert_eq!(None, merge_two_lists(None, None));
        assert_eq!(
            Some(Box::new(ListNode::new(1))),
            merge_two_lists(None, Some(Box::new(ListNode::new(1))))
        );
        assert_eq!(
            from_vec(vec![1, 1, 2, 3, 4, 5]),
            merge_two_lists(from_vec(vec![1, 4, 5]), from_vec(vec![1, 2, 3]))
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
