class Solution:
    def findPermutationDifference(self, s: str, t: str) -> int:

        char_to_index_in_t = {char: idx for idx, char in enumerate(t)}

        permutation_difference = 0

        # Iterate through each character in s and calculate the absolute difference
        for idx, char in enumerate(s):
            diff = abs(idx - char_to_index_in_t[char])
            permutation_difference += diff

        return permutation_difference
