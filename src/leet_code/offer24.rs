use crate::algorithms::solution::ListNode;
use crate::Solution;

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
    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node = None;
        while let Some(current) = head {
            node = Some(Box::new(ListNode {
                val: current.val,
                next: node,
            }));
            head = current.next;
        }
        node
    }
}
