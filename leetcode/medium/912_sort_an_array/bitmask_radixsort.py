"""

https://leetcode.com/problems/sort-an-array/description/

"""


def sort_array(nums: list[int]) -> list[int]:
    l = len(nums)
    mask = 1 << 16
    p = set()
    while mask:
        p0 = 0
        for i in range(l):
            if i in p:
                p.add(p0)
                p0 = i
            if nums[i] & mask == 0:
                nums[p0], nums[i] = nums[i], nums[p0]
                p0 += 1
        p.add(p0)
        mask >>= 1
    ptr = -1
    while nums[ptr]<0:
        ptr -= 1
    return nums[ptr+1:]+nums[:ptr+1]
