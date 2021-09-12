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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;
        let mut prev = head.unwrap();
        let mut curr = prev.next.take();
        while let Some(mut node) = curr {
            curr = node.next.replace(prev);
            prev = node;
        }
        Some(prev)
    }

    pub fn reverse_list_recursive(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn rev(curr: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            if let Some(mut node) = curr {
                let next = node.next;
                node.next = prev;
                rev(next, Some(node))
            } else {
                prev
            }
        }
        rev(head, None)
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let head = list!();
        let res = list!();
        assert_eq!(Solution::reverse_list(head), res);
        let head = list!(1);
        let res = list!(1);
        assert_eq!(Solution::reverse_list(head), res);
        let head = list!(1, 2);
        let res = list!(2, 1);
        assert_eq!(Solution::reverse_list(head), res);
        let head = list!(1, 2, 3);
        let res = list!(3, 2, 1);
        assert_eq!(Solution::reverse_list(head), res);
    }

    #[test]
    fn test_recursive() {
        let head = list!();
        let res = list!();
        assert_eq!(Solution::reverse_list_recursive(head), res);
        let head = list!(1);
        let res = list!(1);
        assert_eq!(Solution::reverse_list_recursive(head), res);
        let head = list!(1, 2);
        let res = list!(2, 1);
        assert_eq!(Solution::reverse_list_recursive(head), res);
        let head = list!(1, 2, 3);
        let res = list!(3, 2, 1);
        assert_eq!(Solution::reverse_list_recursive(head), res);
    }
}
