/*
''''Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

'''
*/
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32
 {
        if needle.is_empty() {
            return 0;
        }
        match haystack.find(&needle) {
            Some(index) => index as i32,
            None => -1,
        }
 }}