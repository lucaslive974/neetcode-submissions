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
            let mut slow = head;
            if slow.is_null() {
                return false;
            }

            let mut fast = (*slow).next;
            if fast.is_null() {
                return false;
            }

            while !fast.is_null() && !(*fast).next.is_null() {
                if slow == fast {
                    return true;
                }

                slow = (*slow).next;
                fast = (*(*fast).next).next;
            }

            return false;
        }
    }
}
