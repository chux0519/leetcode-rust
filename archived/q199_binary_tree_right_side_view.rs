struct Solution;

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
// Definition for a binary tree node.
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }
        let mut view = Vec::new();
        let mut q = VecDeque::new();
        q.push_back(root);
        while !q.is_empty() {
            let level_size = q.len();
            let mut view_of_level = 0;
            for _ in 0..level_size {
                let front = q.pop_front().unwrap();
                match front {
                    Some(node) => {
                        let _node = node.borrow();
                        view_of_level = _node.val;
                        if _node.left.is_some() {
                            q.push_back(_node.left.clone());
                        }
                        if _node.right.is_some() {
                            q.push_back(_node.right.clone());
                        }
                    }
                    None => {}
                }
            }
            view.push(view_of_level);
        }

        view
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q199() {
        let mut root = TreeNode::new(1);
        let mut node2 = TreeNode::new(2);
        let mut node3 = TreeNode::new(3);
        let node4 = TreeNode::new(4);
        let node5 = TreeNode::new(5);
        node2.right = Some(Rc::new(RefCell::new(node5)));
        node3.right = Some(Rc::new(RefCell::new(node4)));
        root.left = Some(Rc::new(RefCell::new(node2)));
        root.right = Some(Rc::new(RefCell::new(node3)));
        let intput = Some(Rc::new(RefCell::new(root)));
        let output = vec![1, 3, 4];
        assert_eq!(output, Solution::right_side_view(intput));
    }
}
