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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::dfs_height(root) != -1
    }

    pub fn dfs_height(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match node {
            Some(_node) => {
                let node_ref = _node.borrow();
                let left_h = Solution::dfs_height(node_ref.left.clone());
                if left_h < 0 {
                    return left_h;
                }
                let right_h = Solution::dfs_height(node_ref.right.clone());
                if right_h < 0 {
                    return right_h;
                }

                if (left_h - right_h).abs() > 1 {
                    return -1;
                }

                return std::cmp::max(left_h, right_h) + 1;
            }
            None => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q110() {
        assert_eq!(true, Solution::is_balanced(None));
        //     3
        //    / \
        //   9  20
        //     /  \
        //    15   7
        let mut root = TreeNode::new(3);
        let node9 = TreeNode::new(9);
        let mut node20 = TreeNode::new(20);
        let node15 = TreeNode::new(15);
        let node7 = TreeNode::new(7);
        node20.left = Some(Rc::new(RefCell::new(node15)));
        node20.right = Some(Rc::new(RefCell::new(node7)));
        root.left = Some(Rc::new(RefCell::new(node9)));
        root.right = Some(Rc::new(RefCell::new(node20)));
        assert_eq!(
            true,
            Solution::is_balanced(Some(Rc::new(RefCell::new(root))))
        );
    }
}
