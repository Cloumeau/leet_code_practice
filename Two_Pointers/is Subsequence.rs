/*
Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).


*/
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        let mut i = 0;
        let mut j = 0;
        while i < s_chars.len() && j < t_chars.len() {
            if s_chars[i] == t_chars[j] {
                i += 1;
            }
            j += 1;
        }
        return i == s_chars.len();
    }
}



