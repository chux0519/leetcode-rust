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
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ret = Vec::new();
        if root.is_none() {
            return ret;
        }
        let mut reverse = false;
        let mut q = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            let mut level = Vec::new();
            for _ in 0..q.len() {
                let front = q.pop_front().unwrap();
                if let Some(f) = front {
                    let f = f.borrow();
                    level.push(f.val);
                    if f.left.is_some() {
                        q.push_back(f.left.clone());
                    }
                    if f.right.is_some() {
                        q.push_back(f.right.clone());
                    }
                }
            }
            if reverse {
                level.reverse();
            }
            ret.push(level);
            reverse = !reverse;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q103() {
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
        let res = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(Solution::zigzag_level_order(root), res);
    }
}
