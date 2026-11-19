// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }
//
// impl ListNode {
//     #[inline]
//     pub fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list1 = list1;
        let mut list2 = list2;

        let mut dummy = ListNode::new(0);
        let mut tail = &mut dummy;
        
        loop {
            match (list1, list2) {
                (Some(mut l1), Some(l2)) if l1.val <= l2.val => {
                    list1 = l1.next.take();

                    list2 = Some(l2);
                    tail.next = Some(l1);
                    
                    tail = tail.next.as_mut().unwrap();
                }
                (Some(l1), Some(mut l2)) if l1.val > l2.val => {
                    list2 = l2.next.take();

                    list1 = Some(l1);
                    tail.next = Some(l2);

                    tail = tail.next.as_mut().unwrap();
                }
                (l1, l2) => {
                    tail.next = l1.or(l2);
                    break;
                }
            }
        }

        dummy.next
    }
}
