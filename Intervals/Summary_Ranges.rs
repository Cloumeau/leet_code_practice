/*
You are given a sorted unique integer array nums.

A range [a,b] is the set of all integers from a to b (inclusive).

Return the smallest sorted list of ranges that cover all the numbers in the array exactly. 
That is, each element of nums is covered by exactly one of the ranges, and there is no 
integer x such that x is in one of the ranges but not in nums.
*/
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        if nums.is_empty() {
            return vec![];
        }
        
        let mut result = Vec::new();
        let mut start = 0usize; // Start index of current range
        
        for i in 0..nums.len() {
            // Check if we're at the end OR next number is not consecutive
            if i == nums.len() - 1 || nums[i] + 1 != nums[i + 1] {
                // If start and current are same, it's a single number
                if start == i {
                    result.push(nums[start].to_string());
                } else {
                    // It's a range
                    result.push(format!("{}->{}", nums[start], nums[i]));
                }
                
                // Move start to next position
                start = i + 1;
            }
        }
        
        result
    }
}