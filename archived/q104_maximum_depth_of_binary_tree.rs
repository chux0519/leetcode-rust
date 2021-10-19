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
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        return Solution::max_depth_recursive(root, 0);
    }

    pub fn max_depth_recursive(root: Option<Rc<RefCell<TreeNode>>>, max_level: i32) -> i32 {
        match root {
            Some(_root) => {
                let root_ref = _root.borrow();
                let cur_level = max_level + 1;
                return std::cmp::max(
                    Solution::max_depth_recursive(root_ref.left.clone(), cur_level),
                    Solution::max_depth_recursive(root_ref.right.clone(), cur_level),
                );
            }
            None => {
                return max_level;
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q104() {
        // [3,9,20,null,null,15,7]
        //      3
        //     / \
        //    9  20
        //      /  \
        //     15   7
        let mut root = TreeNode::new(3);
        let node9 = TreeNode::new(9);
        let mut node20 = TreeNode::new(20);
        let node15 = TreeNode::new(15);
        let node7 = TreeNode::new(7);
        node20.left = Some(Rc::new(RefCell::new(node15)));
        node20.right = Some(Rc::new(RefCell::new(node7)));
        root.left = Some(Rc::new(RefCell::new(node9)));
        root.right = Some(Rc::new(RefCell::new(node20)));

        let input = Some(Rc::new(RefCell::new(root)));
        assert_eq!(3, Solution::max_depth(input));
    }
}
