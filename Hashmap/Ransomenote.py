'''
Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

Each letter in magazine can only be used once in ransomNote.
'''
class Solution:
    def canConstruct(self, ransomNote: str, magazine: str) -> bool:
        # Count frequency of each character in magazine
        magazine_count = {}
        for char in magazine:
            magazine_count[char] = magazine_count.get(char, 0) + 1
        
        # Check if we can construct ransomNote
        for char in ransomNote:
            if magazine_count.get(char, 0) == 0:
                return False
            magazine_count[char] -= 1
        
        return True