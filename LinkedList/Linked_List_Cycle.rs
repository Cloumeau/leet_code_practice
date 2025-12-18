/*
Given head, the head of a linked list, determine if the linked list has a cycle in it.
Return true if there is a cycle in the linked list. Otherwise, return false.

Floyd's Tortoise and Hare - O(n) time, O(1) space
*/

// LeetCode provides this for cycle problems:
// pub struct ListNode { pub val: i32, pub next: *mut ListNode }

impl Solution {
    pub fn has_cycle(head: *mut ListNode) -> bool {
        let mut slow = head;
        let mut fast = head;
        
        unsafe {
            while !fast.is_null() && !(*fast).next.is_null() {
                slow = (*slow).next;
                fast = (*(*fast).next).next;
                
                if slow == fast {
                    return true;
                }
            }
        }
        
        false
    }
}