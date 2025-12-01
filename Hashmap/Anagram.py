'''Given two strings s and t, return true if t is an anagram of s, and false otherwise.
'''
class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        # Different lengths can't be anagrams
        if len(s) != len(t):
            return False
        
        # Count frequency of each character
        char_count = {}
        
        # Add counts from s
        for char in s:
            char_count[char] = char_count.get(char, 0) + 1
        
        # Subtract counts from t
        for char in t:
            if char not in char_count:
                return False
            char_count[char] -= 1
            if char_count[char] < 0:
                return False
        
        return True