'''
You are given a sorted unique integer array nums.

A range [a,b] is the set of all integers from a to b (inclusive).

Return the smallest sorted list of ranges that cover all the numbers in the array exactly. That is, each element of nums is covered by exactly one of the ranges, and there is no integer x such that x is in one of the ranges but not in nums.
'''
class Solution:
    def summaryRanges(self, nums: List[int]) -> List[str]:
        if not nums:
            return []
        
        result = []
        start = 0  # Start of current range
        
        for i in range(len(nums)):
            # Check if we're at the end OR next number is not consecutive
            if i == len(nums) - 1 or nums[i] + 1 != nums[i + 1]:
                # If start and current are same, it's a single number
                if start == i:
                    result.append(str(nums[start]))
                else:
                    # It's a range
                    result.append(f"{nums[start]}->{nums[i]}")
                
                # Move start to next position
                start = i + 1
        
        return result