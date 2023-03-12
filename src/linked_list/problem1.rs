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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub data: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { 
            data: val,
            next: None 
        }
    }
}


pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut node = None;
    
    while let Some(next) = head {
        node = Some(
            Box::new(
                ListNode {
                    data: next.data,
                    next: node
                }
            )
        );
        head = next.next;
    }
    node
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_reverse_linked_list() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

        let new_head = reverse_list(head);

        let mut actual_value = Vec::new();
        let mut current = new_head.as_ref();

        while let Some(node) = current {
            actual_value.push(node.data);
            current = node.next.as_ref();
        }

        assert_eq!(vec![5, 4, 3, 2, 1], actual_value);
    }

    #[test]
    fn test_reverse_linked_lists() {
        let mut head = Some(Box::new(ListNode::new(1)));
        head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
        head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
        head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));

        let new_head = reverse_list(head);

        let mut current = new_head.as_ref();
        assert_eq!(current.unwrap().data, 5);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().data, 4);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().data, 3);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().data, 2);
        current = current.unwrap().next.as_ref();
        assert_eq!(current.unwrap().data, 1);
        current = current.unwrap().next.as_ref();
        assert!(current.is_none());
    }

}