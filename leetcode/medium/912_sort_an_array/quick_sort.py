from random import randint
from collections import deque


def sort_array_recursive(nums: list[int]) -> list[int]:
    def qs(start, end):
        if start >= end - 1:
            return
        for i in range(start, end - 1):
            if nums[i] > nums[i + 1]:
                break
        else:
            return
        p0 = start
        # randomize pivot
        r = randint(start, end - 1)
        nums[r], nums[end - 1] = nums[end - 1], nums[r]
        # pivot is the last element of array,
        pivot = nums[end - 1]
        # loop on hte array except the last element
        for i in range(start, end - 1):
            if nums[i] < pivot:
                nums[p0], nums[i] = nums[i], nums[p0]
                p0 += 1
        # put the last element in its place
        nums[p0], nums[end - 1] = nums[end - 1], nums[p0]
        # recursion
        qs(start, p0)
        qs(p0 + 1, end)

    qs(0, len(nums))
    return nums


def sort_array(nums: list[int]) -> list[int]:
    stack = deque()
    stack.append((0, len(nums)))
    while stack:
        start, end = stack.pop()

        if start >= end - 1:
            continue
        for i in range(start, end - 1):
            if nums[i] > nums[i + 1]:
                break
        else:
            continue
        p0 = start
        # randomize pivot
        r = randint(start, end - 1)
        nums[r], nums[end - 1] = nums[end - 1], nums[r]
        # pivot is the last element of array,
        pivot = nums[end - 1]
        # loop on the array except the last element
        for i in range(start, end - 1):
            if nums[i] < pivot:
                nums[p0], nums[i] = nums[i], nums[p0]
                p0 += 1
        # put the last element in its place
        nums[p0], nums[end - 1] = nums[end - 1], nums[p0]
        # not recursion
        stack.append((start, p0))
        stack.append((p0 + 1, end))
    return nums
