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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut sum: Option<Box<ListNode>> = None;
        let mut p1: &Option<Box<ListNode>> = &l1;
        let mut p2: &Option<Box<ListNode>> = &l2;
        let mut p3: &mut Option<Box<ListNode>> = &mut sum;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry != 0 {
            let mut val = carry;
            if let Some(n1) = p1 {
                val += n1.val;
                p1 = &n1.next;
            }
            if let Some(n2) = p2 {
                val += n2.val;
                p2 = &n2.next;
            }
            carry = val / 10;
            *p3 = Some(Box::new(ListNode {
                val: val % 10,
                next: None,
            }));
            p3 = &mut p3.as_mut().unwrap().next;
        }
        sum
    }
}
//leetcode submit region end(Prohibit modification and deletion)

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let l1 = list!(2, 4, 3);
        let l2 = list!(5, 6, 4);
        let l3 = list!(7, 0, 8);
        assert_eq!(Solution::add_two_numbers(l1, l2), l3);
    }
}
