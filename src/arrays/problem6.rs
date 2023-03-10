/*
        ************* Min Stack *************

Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.

Implement the MinStack class:

MinStack() initializes the stack object.
void push(int val) pushes the element val onto the stack.
void pop() removes the element on the top of the stack.
int top() gets the top element of the stack.
int getMin() retrieves the minimum element in the stack.
You must implement a solution with O(1) time complexity for each function.

 

Example 1:

Input
["MinStack","push","push","push","getMin","pop","top","getMin"]
[[],[-2],[0],[-3],[],[],[],[]]

Output
[null,null,null,null,-3,null,0,-2]

Explanation
MinStack minStack = new MinStack();
minStack.push(-2);
minStack.push(0);
minStack.push(-3);
minStack.getMin(); // return -3
minStack.pop();
minStack.top();    // return 0
minStack.getMin(); // return -2
 

Constraints:

-231 <= val <= 231 - 1
Methods pop, top and getMin operations will always be called on non-empty stacks.
At most 3 * 104 calls will be made to push, pop, top, and getMin.
*/

#[derive(PartialEq, Debug)]
pub struct MinStack {
    pub stack: Vec<i32>,
    pub min_stack: Vec<i32>
}

impl MinStack {
    pub fn new() -> Self {
        Self { 
            stack: vec![], 
            min_stack: vec![] 
        }
    }

    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.min_stack.is_empty() || Some(&val) <= self.min_stack.last() {
            self.min_stack.push(val);
        }
    }

    pub fn pop(&mut self) {
        let val = self.stack.pop().unwrap();
        if Some(&val) == self.min_stack.last() {
            self.min_stack.last().unwrap();
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.last().clone().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        *self.min_stack.last().clone().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_stack_struct() {
        let new_min_stack = MinStack::new();
        let expected_min_stack = MinStack { stack: vec![], min_stack: vec![] };

        assert_eq!(new_min_stack, expected_min_stack);
    }

    #[test]
    fn min_stack_push() {
        let mut new_min_stack = MinStack::new();
        new_min_stack.push(-2);
        new_min_stack.push(0);
        new_min_stack.push(-3);
        let expected_min_stack = MinStack { stack: vec![-2, 0, -3], min_stack: vec![-2, -3] };

        assert_eq!(new_min_stack, expected_min_stack);
    }

    #[test]
    fn min_stack_get_min() {
        let mut new_min_stack = MinStack::new();
        new_min_stack.push(-2);
        new_min_stack.push(0);
        new_min_stack.push(-3);
        
        // get_min()
        let actual_get_min = new_min_stack.get_min();
        let expected_get_min = -3;

        assert_eq!(actual_get_min, expected_get_min);
    }

    #[test]
    fn min_stack_pop_and_top() {
        let mut new_min_stack = MinStack::new();
        new_min_stack.push(-2);
        new_min_stack.push(0);
        new_min_stack.push(-3);
        
        // get_min()
        new_min_stack.get_min();

        // pop()
        new_min_stack.pop();
        // top()
        let actual_pop_and_top = new_min_stack.top();

        let expected_get_min = 0;

        assert_eq!(actual_pop_and_top, expected_get_min);
    }

    #[test]
    fn min_stack_get_min_test() {
        let mut new_min_stack = MinStack::new();
        new_min_stack.push(-2);
        new_min_stack.push(0);
        new_min_stack.push(-3);
        
        // get_min()
        new_min_stack.get_min();  //-3

        // pop()
        new_min_stack.pop();
        // top()
        new_min_stack.top();

        // get_min()
        let actual_get_min = new_min_stack.get_min(); //-2
        let expected_get_min = -3;  //-2

        assert_eq!(actual_get_min, expected_get_min);
    }
}