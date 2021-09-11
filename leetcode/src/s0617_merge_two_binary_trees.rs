//leetcode submit region begin(Prohibit modification and deletion)
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

use rustgym_util::*;

struct Solution;

impl Solution {
    pub fn merge_trees(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1, root2) {
            (Some(node1), Some(node2)) => {
                let mut node1 = node1.borrow_mut();
                let mut node2 = node2.borrow_mut();
                Some(Rc::new(RefCell::new(TreeNode {
                    val: node1.val + node2.val,
                    left: Self::merge_trees(node1.left.take(), node2.left.take()),
                    right: Self::merge_trees(node1.right.take(), node2.right.take()),
                })))
            }
            (None, Some(node2)) => Some(node2),
            (Some(node1), None) => Some(node1),
            _ => None,
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t1 = tree!(1, tree!(3, tree!(5), None), tree!(2));
        let t2 = tree!(2, tree!(1, None, tree!(4)), tree!(3, None, tree!(7)));
        let res = tree!(3, tree!(4, tree!(5), tree!(4)), tree!(5, None, tree!(7)));
        assert_eq!(Solution::merge_trees(t1, t2), res);
    }
}
