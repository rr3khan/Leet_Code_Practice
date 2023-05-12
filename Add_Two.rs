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
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut carry_over: i32 = 0;

        let mut sum_node = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut sum_node;

        let (mut l1, mut l2) = (l1, l2);

        while l1.is_some() || l2.is_some() {

            let val1 = l1.as_ref().map_or(0, |node| node.val);
            let val2 = l2.as_ref().map_or(0, |node| node.val);
            let sum = val1 + val2 + carry_over;
            carry_over = sum / 10;

            tail = &mut tail.as_mut().unwrap().next;
            *tail = Some(Box::new(ListNode::new(sum % 10)));

            l1 = l1.and_then(|node| node.next);
            l2 = l2.and_then(|node| node.next);
        }

        if (carry_over != 0){
            tail = &mut tail.as_mut().unwrap().next;
            *tail = Some(Box::new(ListNode::new(carry_over)));
        }

    return sum_node.unwrap().next;
        
    }
}