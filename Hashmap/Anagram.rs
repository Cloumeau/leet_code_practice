use std::collections::HashMap;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        
        let mut char_count: HashMap<char, i32> = HashMap::new();
        
        // Count characters in s
        for c in s.chars() {
            *char_count.entry(c).or_insert(0) += 1;
        }
        
        // Subtract characters in t
        for c in t.chars() {
            *char_count.entry(c).or_insert(0) -= 1;
            if char_count[&c] < 0 {
                return false;
            }
        }
        
        true
    }
}