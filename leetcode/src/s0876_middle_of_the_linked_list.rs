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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = head.clone();
        let mut fast = head;
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            fast = fast.unwrap().next.unwrap().next;
            slow = slow.unwrap().next;
        }
        slow
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let head = list![1, 2, 3, 4, 5];
        let middle = list![3, 4, 5];
        assert_eq!(Solution::middle_node(head), middle);

        let head = list![1, 2, 3, 4, 5, 6];
        let middle = list![4, 5, 6];
        assert_eq!(Solution::middle_node(head), middle);
    }
}
