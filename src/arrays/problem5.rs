/*
        ************* Valid Parentheses *************

Given a string "s" containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.
 

Example 1:

Input: s = "()"
Output: true
Example 2:

Input: s = "()[]{}"
Output: true
Example 3:

Input: s = "(]"
Output: false
 

Constraints:

1 <= s.length <= 104
s consists of parentheses only '()[]{}'.
*/


use std::collections::HashMap;

pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let operations = HashMap::from([(']', '['), ('}', '{'), (')', '(')]);

    for c in s.chars() {
        match c {
            '(' => stack.push(c),
            '{' => stack.push(c),
            '[' => stack.push(c),
            _ => {
                if stack.iter().last() == operations.get(&c) {
                    stack.pop();
                } else {
                    return false;
                }
            }
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkin_valid() {
        let s = "()";
        let actual_valid = is_valid(s.to_string());
        let expected_valid = true;
        assert_eq!(actual_valid, expected_valid);
    }

    #[test]
    fn checkin_valid_true() {
        let s = "()[]{}";
        let actual_valid = is_valid(s.to_string());
        let expected_valid = true;
        assert_eq!(actual_valid, expected_valid);
    }

    #[test]
    fn checkin_valid_false() {
        let s = "(]";
        let actual_valid = is_valid(s.to_string());
        let expected_valid = false;
        assert_eq!(actual_valid, expected_valid);
    }
}