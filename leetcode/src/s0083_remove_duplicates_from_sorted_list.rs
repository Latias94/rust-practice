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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head; // 避免参数上加 mut
        let mut cur = head.as_mut();
        while let Some(mut node) = cur.take() {
            while let Some(next) = node.next.as_mut() {
                if node.val != next.val {
                    break;
                }
                node.next = next.next.take();
            }
            cur = node.next.as_mut();
        }
        head
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let p = list!(1, 1, 2);
        let q = list!(1, 2);
        assert_eq!(Solution::delete_duplicates(p), q);
        let p = list!(1, 1, 2, 3, 3);
        let q = list!(1, 2, 3);
        assert_eq!(Solution::delete_duplicates(p), q);
    }
}
