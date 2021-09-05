struct Solution;
use rustgym_util::*;

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

trait Height {
    fn height(&self) -> u8;
}

impl Height for Option<Rc<RefCell<TreeNode>>> {
    fn height(&self) -> u8 {
        match self {
            None => 0,
            Some(node) => {
                let node = node.borrow();
                u8::max(node.left.height(), node.right.height()) + 1
            }
        }
    }
}

impl Solution {
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                let node = node.borrow();
                let left_height = node.left.height();
                let right_height = node.right.height();
                if left_height == right_height
                    || left_height + 1 == right_height
                    || right_height + 1 == left_height
                {
                    Self::is_balanced(node.left.clone()) && Self::is_balanced(node.right.clone())
                } else {
                    false
                }
            }
        }
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let a = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
        let res = true;
        assert_eq!(Solution::is_balanced(a), res);

        let b = tree!(
            1,
            tree!(2, tree!(3, tree!(4), tree!(4)), tree!(3)),
            tree!(2)
        );
        let res = false;
        assert_eq!(Solution::is_balanced(b), res);
    }
}
