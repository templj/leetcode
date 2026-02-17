from typing import List


class Solution:
    def alternatingSum(self, nums: List[int]) -> int:
        out = 0
        for i, num in enumerate(nums):
            if i % 2 == 0:
                out += num
            else:
                out -= num
        return out
