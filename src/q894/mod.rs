// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
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

/// FBT(N) = [All trees with left child from FBT(x) and right child from FBT(N−1−x), for all x].
///
/// ```rust
/// let mut n1 = TreeNode::new(0);
/// let n2 = TreeNode::new(0);
/// let n3 = TreeNode::new(0);
/// n1.left = Some(Rc::new(RefCell::new(n2)));
/// n1.right = Some(Rc::new(RefCell::new(n3)));
///
/// assert_eq!(
///     vec![Some(Rc::new(RefCell::new(n1)))],
///     Solution::all_possible_fbt(3)
/// );
/// ```
struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut hash = HashMap::new();
        let list = vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))];
        hash.insert(1, list);

        Solution::all_possible_fbt_rec(n, &mut hash)
    }

    pub fn all_possible_fbt_rec(
        n: i32,
        hash: &mut HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>>,
    ) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        match hash.get(&n) {
            None => {
                let mut ret = Vec::new();
                if n == 1 {
                    ret.push(Some(Rc::new(RefCell::new(TreeNode::new(0)))));
                } else if n % 2 == 1 {
                    for x in 0..n {
                        let y = n - 1 - x;
                        for left in Solution::all_possible_fbt_rec(x, hash) {
                            for right in Solution::all_possible_fbt_rec(y, hash) {
                                let mut node = TreeNode::new(0);
                                node.left = left.clone();
                                node.right = right;
                                ret.push(Some(Rc::new(RefCell::new(node))));
                            }
                        }
                    }
                }
                ret
            }
            Some(v) => v.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn q894() {
        //   0
        // 0   0
        let mut n1 = TreeNode::new(0);
        let n2 = TreeNode::new(0);
        let n3 = TreeNode::new(0);
        n1.left = Some(Rc::new(RefCell::new(n2)));
        n1.right = Some(Rc::new(RefCell::new(n3)));

        assert_eq!(
            vec![Some(Rc::new(RefCell::new(n1)))],
            Solution::all_possible_fbt(3)
        );
    }
}
