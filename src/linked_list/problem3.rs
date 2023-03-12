/*
        ************* Merge Two Sorted Lists *************

You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the 
first two lists.

Return the head of the merged linked list.

Examples:

Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]
Example 2:

Input: list1 = [], list2 = []
Output: []
Example 3:

Input: list1 = [], list2 = [0]
Output: [0]
 

Constraints:

The number of nodes in both lists is in the range [0, 50].
-100 <= Node.val <= 100
Both list1 and list2 are sorted in non-decreasing order.
*/

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    let mut tail = &mut dummy;

    while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
        if node1.val <= node2.val {
            tail.as_mut().unwrap().next = Some(node1.clone());
            list1 = node1.next.clone();
        } else {
            tail.as_mut().unwrap().next = Some(node2.clone());
            list2 = node2.next.clone();
        }
        tail = &mut tail.as_mut().unwrap().next;
    }

    tail.as_mut().unwrap().next = list1.or(list2);

    dummy.unwrap().next

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn got_merged() {
        let mut l1 = Some(Box::new(ListNode::new(1)));
    l1.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    l1.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let mut l2 = Some(Box::new(ListNode::new(1)));
    l2.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    l2.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));

    let merged_list = merge_two_lists(l1, l2);

    let mut actual_value = Vec::new();
    let mut current = merged_list.as_ref();

    while let Some(node) = current {
        actual_value.push(node.val);
        current = node.next.as_ref();
    }

    assert_eq!(vec![1, 1, 2, 3, 4, 4], actual_value);

    }
}