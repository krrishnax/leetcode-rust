/*
        ************* Reverse Linked List *************

Given the head of a singly linked list, reverse the list, and return the reversed list.

Example 1:
Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]
Example 2:

Input: head = [1,2]
Output: [2,1]
Example 3:

Input: head = []
Output: []
 

Constraints:

The number of nodes in the list is the range [0, 5000].
-5000 <= Node.val <= 5000
 

Follow up: A linked list can be reversed either iteratively or recursively. Could you implement both? 
*/

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}


pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;

    while let Some(mut boxed_node) = curr.take() {
        let next = boxed_node.next.take();
        boxed_node.next = prev;
        prev = Some(boxed_node);
        curr = next;
    }

    prev
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_linked_list() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

        let new_head = reverse_list(head);

        let mut current = new_head.as_ref();
        assert_eq!(current.unwrap().val, 5);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().val, 4);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().val, 3);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().val, 2);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().val, 1);
        current = current.unwrap().next.as_ref();
        assert!(current.is_none());
    }

    #[test]
    fn test_reverse_linked_lists() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

        let new_head = reverse_list(head);

        let mut actual_value = Vec::new();
        let mut current = new_head.as_ref();

        while let Some(node) = current {
            actual_value.push(node.val);
            current = node.next.as_ref();
        }

        assert_eq!(vec![5, 4, 3, 2, 1], actual_value);
    }
}