struct Solution;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut depth = 0;
        if root.is_none() {
            return depth;
        }
        let mut q = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            // each level
            depth += 1;
            for _ in 0..q.len() {
                let front = q.pop_front().unwrap();
                match front {
                    Some(n) => {
                        let node = n.borrow();
                        if node.left.is_none() && node.right.is_none() {
                            return depth;
                        } else {
                            if node.left.is_some() {
                                q.push_back(node.left.clone());
                            }
                            if node.right.is_some() {
                                q.push_back(node.right.clone());
                            }
                        }
                    }
                    None => unreachable!(),
                }
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q111() {
        {
            let mut node3 = TreeNode::new(3);
            let node9 = TreeNode::new(9);
            let mut node20 = TreeNode::new(20);
            let node15 = TreeNode::new(15);
            let node7 = TreeNode::new(7);
            node20.left = Some(Rc::new(RefCell::new(node15)));
            node20.right = Some(Rc::new(RefCell::new(node7)));
            node3.left = Some(Rc::new(RefCell::new(node9)));
            node3.right = Some(Rc::new(RefCell::new(node20)));

            let root = Some(Rc::new(RefCell::new(node3)));
            assert_eq!(Solution::min_depth(root), 2);
        }
        {
            let mut node1 = TreeNode::new(1);
            let mut node2 = TreeNode::new(2);
            let node3 = TreeNode::new(3);
            let node4 = TreeNode::new(4);
            let node5 = TreeNode::new(5);
            node2.left = Some(Rc::new(RefCell::new(node4)));
            node2.right = Some(Rc::new(RefCell::new(node5)));
            node1.left = Some(Rc::new(RefCell::new(node2)));
            node1.right = Some(Rc::new(RefCell::new(node3)));

            let root = Some(Rc::new(RefCell::new(node1)));
            assert_eq!(Solution::min_depth(root), 2);
        }
    }
}
