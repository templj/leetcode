#include <stdio.h>

int minOperations(int* nums, int numsSize, int k) {
    int sum = 0;
    for (int i = 0; i < numsSize; i++) {
        sum += nums[i];
    }
    if (sum % k == 0) {
        return 0;
    } else if (sum < k) {
        return sum;
    } else {
        return sum % k;
    }
    return 0;
}

int main() {
    int nums[] = {3,9,7};
    int k = 5;
    int result = minOperations(nums, sizeof(nums) / sizeof(nums[0]), k);
    printf("Result: %d\n", result);

    int nums2[] = {4,1,3};
    int k2 = 4;
    int result2 = minOperations(nums2, sizeof(nums2) / sizeof(nums2[0]), k2);
    printf("Result: %d\n", result2);

    int nums3[] = {3,2};
    int k3 = 6;
    int result3 = minOperations(nums3, sizeof(nums3) / sizeof(nums3[0]), k3);
    printf("Result: %d\n", result3);

    return 0;
}
