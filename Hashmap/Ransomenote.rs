use std::collections::HashMap;

/*
Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

Each letter in magazine can only be used once in ransomNote.
*/
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_count = HashMap::new();
        for ch in magazine.chars() {
            *magazine_count.entry(ch).or_insert(0) += 1;
        }
        
        for ch in ransom_note.chars() {
            let count = magazine_count.entry(ch).or_insert(0);
            if *count == 0 {
                return false;
            }
            *count -= 1;
        }
        
        true
    }
}