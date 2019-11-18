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

// solution start here
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;
impl Solution {
    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 容易想到的是递归解法，即左子树和右子树递归比较
        // Solution::symmetric(root.clone(), root)
        // 另一个解法是利用队列，避免递归
        match root {
            Some(r) => {
                return Solution::symmetric_queue(
                    r.borrow().left.clone(),
                    r.borrow().right.clone(),
                );
            }
            None => {
                return true;
            }
        }
    }

    fn symmetric(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if left.is_none() && right.is_none() {
            return true;
        }
        match left {
            Some(l) => match right {
                Some(r) => {
                    let l_inner = l.borrow();
                    let r_inner = r.borrow();
                    dbg!(l_inner.val, r_inner.val);
                    if l_inner.val == r_inner.val {
                        return Solution::symmetric(l_inner.left.clone(), r_inner.right.clone())
                            && Solution::symmetric(l_inner.right.clone(), r_inner.left.clone());
                    }
                    return false;
                }
                None => {
                    return false;
                }
            },
            None => match right {
                Some(r) => {
                    return false;
                }
                None => {
                    return true;
                }
            },
        }
    }
    fn symmetric_queue(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        let mut v = VecDeque::new();
        v.push_back(left);
        v.push_back(right);

        loop {
            if v.is_empty() {
                break;
            }
            if let Some(l) = v.pop_front() {
                if let Some(r) = v.pop_front() {
                    match l {
                        Some(_l) => match r {
                            Some(_r) => {
                                let borrow_l = _l.borrow();
                                let borrow_r = _r.borrow();
                                if borrow_l.val != borrow_r.val {
                                    return false;
                                } else {
                                    // ll, rr and lr rl
                                    v.push_back(borrow_l.left.clone());
                                    v.push_back(borrow_r.right.clone());
                                    v.push_back(borrow_l.right.clone());
                                    v.push_back(borrow_r.left.clone());
                                }
                            }
                            None => {
                                return false;
                            }
                        },
                        None => match r {
                            Some(_r) => {
                                return false;
                            }
                            None => {
                                continue;
                            }
                        },
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q101() {
        let mut node1 = TreeNode::new(1);
        {
            // left
            let mut node2 = TreeNode::new(2);
            let node3 = TreeNode::new(3);
            let node4 = TreeNode::new(4);
            node2.left = Some(Rc::new(RefCell::new(node3)));
            node2.right = Some(Rc::new(RefCell::new(node4)));
            node1.left = Some(Rc::new(RefCell::new(node2)));
        }

        {
            // right
            let mut node2 = TreeNode::new(2);
            let node3 = TreeNode::new(3);
            let node4 = TreeNode::new(4);
            node2.right = Some(Rc::new(RefCell::new(node3)));
            node2.left = Some(Rc::new(RefCell::new(node4)));
            node1.right = Some(Rc::new(RefCell::new(node2)));
        }
        let root = Some(Rc::new(RefCell::new(node1)));
        assert_eq!(Solution::is_symmetric(root), true);
    }
}
