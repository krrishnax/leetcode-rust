/*
        ************* Implement Stack using Queues *************

Implement a last-in-first-out (LIFO) stack using only two queues. The implemented stack should 
support all the functions of a normal stack (push, top, pop, and empty).

Implement the MyStack class:

void push(int x) Pushes element x to the top of the stack.
int pop() Removes the element on the top of the stack and returns it.
int top() Returns the element on the top of the stack.
boolean empty() Returns true if the stack is empty, false otherwise.
Notes:

You must use only standard operations of a queue, which means that only push to back, 
peek/pop from front, size and is empty operations are valid.
Depending on your language, the queue may not be supported natively. You may simulate a 
queue using a list or deque (double-ended queue) as long as you use only a queue's standard operations. 
*/

use std::collections::VecDeque;

pub struct MyStack {
    q: VecDeque<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    pub fn new() -> Self {
        Self { q: VecDeque::new() }
    }

    pub fn push(&mut self, x: i32) {
        self.q.push_back(x);
    }

    pub fn pop(&mut self) -> i32 {
        for _ in 0..self.q.len() - 1 {
            let temp = self.q.pop_front().unwrap();
            self.q.push_back(temp);
        }
        self.q.pop_front().unwrap()
    }

    pub fn top(&self) -> i32 {
        self.q[self.q.len() - 1]
    }

    pub fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

/*
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

 #[cfg(test)]
 mod tests {
     use super::*;
 
     #[test]
     fn test_push_and_pop() {
         let mut stack = MyStack::new();
         stack.push(1);
         stack.push(2);
         stack.push(3);
         assert_eq!(stack.pop(), 3);
         assert_eq!(stack.pop(), 2);
         assert_eq!(stack.pop(), 1);
         assert!(stack.empty());
     }
 
     #[test]
     fn test_top() {
         let mut stack = MyStack::new();
         stack.push(1);
         stack.push(2);
         stack.push(3);
         assert_eq!(stack.top(), 3);
         stack.pop();
         assert_eq!(stack.top(), 2);
         stack.pop();
         assert_eq!(stack.top(), 1);
         stack.pop();
         assert!(stack.empty());
     }
 
     #[test]
     fn test_empty() {
         let stack = MyStack::new();
         assert!(stack.empty());
         let mut stack = MyStack::new();
         stack.push(1);
         assert!(!stack.empty());
     }
 }
 