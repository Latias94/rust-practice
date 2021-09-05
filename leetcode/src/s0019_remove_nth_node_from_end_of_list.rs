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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut pre_head = Some(Box::new(ListNode { val: 0, next: head }));
        let mut fast = pre_head.clone();
        let mut slow = &mut pre_head;
        for _ in 0..n {
            fast = fast.unwrap().next;
        }
        while fast.as_ref().unwrap().next.is_some() {
            fast = fast.unwrap().next;
            slow = &mut slow.as_mut().unwrap().next;
        }
        slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.clone();
        pre_head.unwrap().next
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let head = list![1, 2, 3, 4, 5];
        let res = list![1, 2, 3, 5];
        let n = 2;
        assert_eq!(Solution::remove_nth_from_end(head, n), res);
    }
}
