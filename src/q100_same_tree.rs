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
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        // 进行同顺序的遍历即可
        match p {
            Some(_p) => match q {
                Some(_q) => {
                    let _p_ref = _p.borrow();
                    let _q_ref = _q.borrow();
                    if _p_ref.val != _q_ref.val {
                        return false;
                    } else {
                        // recursive
                        return Solution::is_same_tree(_p_ref.left.clone(), _q_ref.left.clone())
                            && Solution::is_same_tree(_p_ref.right.clone(), _q_ref.right.clone());
                    }
                }
                None => {
                    return false;
                }
            },
            None => match q {
                Some(_q) => {
                    return false;
                }
                None => {}
            },
        }

        true
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q100() {
        let mut node1 = TreeNode::new(1);
        let node2 = TreeNode::new(2);
        let node3 = TreeNode::new(3);
        node1.left = Some(Rc::new(RefCell::new(node2)));
        node1.right = Some(Rc::new(RefCell::new(node3)));

        let p = Some(Rc::new(RefCell::new(node1)));
        let q = p.clone();
        assert_eq!(true, Solution::is_same_tree(p, q));
    }
}
