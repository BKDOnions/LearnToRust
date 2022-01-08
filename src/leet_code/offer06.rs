use crate::algorithms::solution::ListNode;
use crate::Solution;

impl Solution {
    pub fn reverse_print(mut head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res = std::collections::VecDeque::new();
        while let Some(node) = head {
            res.push_front(node.val);
            head = node.next;
        }
        Vec::from(res)
    }
}
