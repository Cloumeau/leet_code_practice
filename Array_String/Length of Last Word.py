'''
Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal substring consisting of non-space characters only.

'''
class Solution:
    def lengthOfLastWord(self, s: str) -> int:
        words = s.split()
        return(len(words[-1]))