'''
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.
"()[]{}"
'''
class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        closing_to_opening = {
            "}": "{",
            "]": "[",
            ")": "("
        }
        for char in s:
            if char in closing_to_opening:
                if not stack or stack[-1] != closing_to_opening[char]:
                    return False
                stack.pop()
            else:
                stack.append(char)
        return len(stack) == 0