/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', 
determine if the input string is valid.

An input string is valid if:
- Open brackets must be closed by the same type of brackets.
- Open brackets must be closed in the correct order.
- Every close bracket has a corresponding open bracket of the same type.
*/
use std::collections::HashMap;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        
        // Map closing brackets to their matching opening brackets
        let closing_to_opening: HashMap<char, char> = HashMap::from([
            ('}', '{'),
            (']', '['),
            (')', '('),
        ]);
        
        for char in s.chars() {
            if let Some(&expected_open) = closing_to_opening.get(&char) {
                // It's a closing bracket - check if it matches
                if stack.is_empty() || stack.last() != Some(&expected_open) {
                    return false;
                }
                stack.pop();
            } else {
                // It's an opening bracket - push to stack
                stack.push(char);
            }
        }
        
        stack.is_empty()
    }
}