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
use std::collections::HashMap;
use std::rc::Rc;

impl Solution {
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let n = preorder.len();
        let map = inorder
            .iter()
            .enumerate()
            .map(|(i, v)| (*v, i))
            .collect::<HashMap<i32, usize>>();
        Self::helper(&preorder, 0, n, &inorder, 0, n as i32, &map)
    }

    pub fn helper(
        preorder: &[i32],
        pre_start: usize,
        pre_end: usize,
        inorder: &[i32],
        in_start: usize,
        in_end: i32,
        map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pre_start > pre_end || in_start as i32 > in_end {
            None
        } else {
            let n = preorder.len();
            if n == 0 || pre_start >= n {
                None
            } else {
                let in_center_val = preorder[pre_start];
                let in_center_index = map.get(&in_center_val).unwrap();
                let num_left = *in_center_index - in_start;

                Some(Rc::new(RefCell::new(TreeNode {
                    val: in_center_val,
                    left: Self::helper(
                        preorder,
                        pre_start + 1,
                        pre_start + num_left,
                        inorder,
                        in_start,
                        *in_center_index as i32 - 1, // usize 的 0 - 1 会 out of bound
                        map,
                    ),
                    right: Self::helper(
                        preorder,
                        pre_start + num_left + 1,
                        pre_end,
                        inorder,
                        in_center_index + 1,
                        in_end,
                        map,
                    ),
                })))
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
        let preorder = vec![3, 9, 20, 15, 7];
        let inorder = vec![9, 3, 15, 20, 7];
        let res = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
        assert_eq!(Solution::build_tree(preorder, inorder), res);
    }
}
