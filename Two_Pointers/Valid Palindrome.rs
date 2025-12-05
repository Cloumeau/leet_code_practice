/*A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.
*/
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut New_s = String::new();
        for char in s.chars() {
            if char.is_alphanumeric() {
                New_s.push(char);
            }
        }
        New_s == New_s.chars().rev().collect::<String>()
    }
}