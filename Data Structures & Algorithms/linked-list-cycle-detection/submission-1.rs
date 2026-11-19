// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: *mut ListNode,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: std::ptr::null_mut(), val }
//     }
// }

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        unsafe {
            let mut slow = head.as_ref();
            let mut fast = slow.and_then(|node| node.next.as_ref());

            while let (Some(s), Some(f)) = (slow, fast) {
                if std::ptr::eq(s, f) {
                    return true;
                }

                slow = s.next.as_ref();
                fast = f.next.as_ref().and_then(|next_node| next_node.next.as_ref());
            }

            false
        }
    }
}
