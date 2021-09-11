struct Solution;

use rustgym_util::*;
//leetcode submit region begin(Prohibit modification and deletion)
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (node1, None) => node1,
            (None, node2) => node2,
            (Some(mut node1), Some(mut node2)) => {
                if node1.val > node2.val {
                    std::mem::swap(&mut node1, &mut node2);
                }
                node1.next = Solution::merge_two_lists(Some(node2), node1.next);
                Some(node1)
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
        let a = list!(1, 2, 4);
        let b = list!(1, 3, 4);
        let c = list!(1, 1, 2, 3, 4, 4);
        assert_eq!(Solution::merge_two_lists(a, b), c);
    }
}
