"""

https://leetcode.com/problems/sort-an-array/description/

"""

from .bitmask_radixsort import sort_array as rdx_sort
from .count_sort import sort_array as cnt_sort
from .quick_sort import sort_array_recursive as rq_sort, sort_array as q_sort
from random import randint
import timeit


def test():
    list_to_sort = [randint(-50_000, 50_000) for _ in range(1_000_000)]
    sorted_list = sorted(list_to_sort)
    print(
        "radix sort             :",
        timeit.timeit(lambda: rdx_sort(list(list_to_sort)), number=1),
    )
    print(
        "count sort             :",
        timeit.timeit(lambda: cnt_sort(list(list_to_sort)), number=1),
    )
    print(
        "quick sort (recursive) :",
        timeit.timeit(lambda: rq_sort(list(list_to_sort)), number=1),
    )
    print(
        "quick sort             :",
        timeit.timeit(lambda: q_sort(list(list_to_sort)), number=1),
    )
    assert rdx_sort(list(list_to_sort)) == sorted_list
    assert cnt_sort(list(list_to_sort)) == sorted_list
    assert rq_sort(list(list_to_sort)) == sorted_list
    assert q_sort(list(list_to_sort)) == sorted_list


if __name__ == "__main__":
    test()
