"""

https://leetcode.com/problems/sort-an-array/description/

"""


def sort_array(nums: list[int]) -> list[int]:
    cnt = [0] * (10**5 + 1)
    for n in nums:
        cnt[n] += 1
    ptr = 0
    for n in range(-50000, 50001):
        for _ in range(cnt[n]):
            nums[ptr] = n
            ptr += 1
    return nums
