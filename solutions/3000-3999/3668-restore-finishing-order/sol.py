from typing import List


class Solution:
    def recoverOrder(self, order: List[int], friends: List[int]) -> List[int]:
        results = []
        for i in range(len(order)):
            if order[i] in friends:
                results.append(order[i])
        return results


order = [3, 1, 2, 5, 4]
friends = [1, 3, 4]
print(Solution().recoverOrder(order, friends))
