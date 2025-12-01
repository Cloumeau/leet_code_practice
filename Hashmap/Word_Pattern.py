class Solution:
    def wordPattern(self, pattern: str, s: str) -> bool:
        words = s.split()
        
        # Check if lengths match
        if len(pattern) != len(words):
            return False
        
        # Two maps: pattern->word and word->pattern
        pattern_to_word = {}
        word_to_pattern = {}
        
        for i in range(len(pattern)):
            char = pattern[i]
            word = words[i]
            
            # Check if pattern->word mapping is consistent
            if char in pattern_to_word:
                if pattern_to_word[char] != word:
                    return False
            else:
                pattern_to_word[char] = word
            
            # Check if word->pattern mapping is consistent
            if word in word_to_pattern:
                if word_to_pattern[word] != char:
                    return False
            else:
                word_to_pattern[word] = char
        
        return True