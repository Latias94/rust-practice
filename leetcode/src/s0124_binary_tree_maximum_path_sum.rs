struct Solution;

use rustgym_util::TreeNode;

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
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_sum = i32::MIN;
        Self::helper(&root, &mut max_sum);
        max_sum
    }

    fn helper(root: &Option<Rc<RefCell<TreeNode>>>, max_sum: &mut i32) -> i32 {
        if let Some(root) = root {
            let node = root.borrow();
            let left = max(0, Self::helper(&node.left, max_sum));
            let right = max(0, Self::helper(&node.right, max_sum));
            *max_sum = max(*max_sum, left + right + node.val);
            return max(left, right) + node.val;
        }
        0
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;
    use rustgym_util::*;

    #[test]
    fn test() {
        let root = tree!(1, tree!(2), tree!(3));
        let res = 6;
        assert_eq!(Solution::max_path_sum(root), res);
        let root = tree!(-10, tree!(9), tree!(20, tree!(15), tree!(7)));
        let res = 42;
        assert_eq!(Solution::max_path_sum(root), res);
    }
}
