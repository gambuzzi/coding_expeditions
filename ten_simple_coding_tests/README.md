# 10 Algorithms To Solve Before your Coding Interview

Inspired by this article

https://towardsdatascience.com/10-algorithms-to-solve-before-your-python-coding-interview-feb74fb9bc27

## 1. Reverse Integer
Given an integer, return the integer with reversed digits.

Note: The integer could be either positive or negative.

* `-231` => `-132`
* `345` => `543`


<details>
	<summary>Solution</summary>

```python
def solution(n):
    return f'-{str(n)[1:][::-1]}' if n<0 else str(n)[::-1]
```

</details>

## 2. Average Words Length

For a given sentence, return the average word length.

Note: Remember to remove punctuation first.


```python
sentence1 = "Hi all, my name is Tom...I am originally from Australia."
sentence2 = "I need to work very hard to learn more about algorithms in Python!"

assert solution(sentence1)==3.82
assert solution(sentence2)==4.08
```

<details>
	<summary>Solution</summary>

```python
import re
def solution(sentence):
    def avg(iterable):
        ret = 0
        n = 0.0  # if using python 2 force to float
        for x in iterable:
            ret+=x
            n+=1.0  # counting like this because `iterable` is a generator and generators has no `len` method
        return ret/n  # if using python 2 remember to cast to float

    return round(avg(map(len, re.findall(r'\w+', sentence))), 2)
```

</details>

## 3. Add Strings
Given two non-negative integers num1 and num2 represented as string, return the sum of num1 and num2.

You must **not** use any built-in BigInteger library or convert the inputs to integer directly.

<details>
	<summary>Solution</summary>

```python
from functools import reduce  # only needed for python 3

def solution(num1, num2):
    n1 = reduce(lambda a, b: a*10+b-48, map(ord, num1), 0)
    n2 = reduce(lambda a, b: a*10+b-48, map(ord, num2), 0)

    return n1+n2
```

</details>

## 4. First Unique Character
Given a string, find the first non-repeating character in it and return its index.

If it doesn't exist, return -1.

*`Note: all the input strings are already lowercase.`*

<details>
	<summary>Solution</summary>

```python
def solution(s):
    seen = [None]*256  # let's assume only ascii characters 0..255
    for i, x in enumerate(s):
        n = ord(x)
        if seen[n] is None:
            seen[n] = i
        elif seen[n] is not False:
            seen[n] = False
    try:
        return min(x for x in seen if x not in (False, None))
    except ValueError:  # empty generator above
        return -1
```

### 2023 08 update

```python
def solution(s):
    seen = [None]*256  # let's assume only ascii characters 0..255
    for i, x in enumerate(s):
        n = ord(x)
        if seen[n] is None:
            seen[n] = i+1
        elif seen[n] is not False:
            seen[n] = False

    return min((x-1 for x in seen if x), default=-1)
```


</details>

## 5. Valid Palindrome

Given a non-empty string s, you may delete at most one character. Judge whether you can make it a palindrome.

The string will only contain lowercase characters a-z.

<details>
	<summary>Solution</summary>

```python
def solution(s):
    if s[:len(s)//2+1] == s[:len(s)//2-1:-1]:
        return True  # already a palindrome
    for i in range(len(s)):
        w = s[:i] + s[i+1:]
        if w[:len(w)//2+1] == w[:len(w)//2-1:-1]:
            return True
    return False
```

</details>

## 6. Monotonic Array

Given an array of integers, determine whether the array is monotonic or not.

```python
A = [6,5,4,4]
B = [1,1,1,3,3,4,3,2,4,2]
C = [1,1,2,3,7]

assert solution(A)
assert not solution(B)
assert solution(C)
```

<details>
	<summary>Solution</summary>

```python
def solution(nums):
    return all(a>=b for a, b in zip(nums, nums[1:])) or \
           all(a<=b for a, b in zip(nums, nums[1:]))

```

### 2023 08 update

```python
from itertools import pairwise

def solution(nums):
    return all(a>=b for a, b in pairwise(nums)) or \
           all(a<=b for a, b in pairwise(nums))

```



</details>

## 7. Move Zeroes

Given an array nums, write a function to move all zeroes to the end of it while maintaining the relative order of the non-zero elements.

```python
array1 = [0,1,0,3,12]
array2 = [1,7,0,0,8,0,10,12,0,4]

assert solution(array1) == [1, 3, 12, 0, 0]
assert solution(array2) == [1, 7, 8, 10, 12, 4, 0, 0, 0, 0]
```

<details>
	<summary>Solution</summary>

```python
def solution(nums):  # NOT changing the input
    ret = [x for x in nums if x!=0]
    return ret + [0] * (len(nums)-len(ret))


from collections import deque
def solution_2(nums):  # changing the input (inplace)
    zeros = deque()
    for i in range(len(nums)):
        x = nums[i]
        if x==0:
            zeros.append(i)
        elif len(zeros)>0:
            idx = zeros.popleft()
            nums[idx] = x
            nums[i] = 0
            zeros.append(i)
    return nums
```

</details>

## 8 Fill The Blanks

Given an array containing `None` values fill in the `None` values with most recent `non-None` value in the array

```python
array1 = [1, None, 2, 3, None, None, 5, None]

assert solution(array1) == [1, 1, 2, 3, 3, 3, 5, 5]
```

<details>
	<summary>Solution</summary>


```python
def solution(arr):
    ret = []
    for x in arr:
        valid = valid if x is None else x
        ret.append(valid)
    return ret

```

</details>

<details>
	<summary>Another solution</summary>

```python

def first_true(iter):
	for x in iter:
		if x is not None:
			return x
	raise ValueError("All the values are None")

def solution(a):
	return [first_true(x) for x in zip(*(([None] * i + a[:len(a)-i]) for i in range(len(a))))]
```

That can become even more obscure

```python
def solution(a):
    return [
        next(y for y in x if y is not None)
        for x in zip(*(([None] * i + a[:len(a)-i]) for i in range(len(a))))
	]

```

Basically I am creating a matrix and rotating it with `zip`. Then for each column (that now is a row), taking the first not `None` element.

### Performances

The first solution is the more readable and the fastest one.

```python
>>> timeit.timeit('solution([1, None, 2, 3, None, None, 5, None])', '''
def solution(arr):
    ret = []
    for x in arr:
        valid = valid if x is None else x
        ret.append(valid)
    return ret
''')
0.6686493580054957
>>> timeit.timeit('solution([1, None, 2, 3, None, None, 5, None])', '''
def first_true(iter):
	for x in iter:
		if x is not None:
			return x
	raise ValueError("All the values are None")

def solution(a):
	return [first_true(x) for x in zip(*(([None] * i + a[:len(a)-i]) for i in range(len(a))))]
''')
4.560916772999917

>>> timeit.timeit('solution([1, None, 2, 3, None, None, 5, None])', '''
def solution(a):
    return [
        next(y for y in x if y is not None)
        for x in zip(*(([None] * i + a[:len(a)-i]) for i in range(len(a))))
	]	
''')
5.971725533992867
```

</details>

## 9. Matched & Mismatched Words

Given two sentences, return an array that has the words that appear in one sentence and not the other and an array with the words in common.

```python
sentence1 = 'We are really pleased to meet you in our city'
sentence2 = 'The city was hit by a really heavy storm'

assert solution(sentence1, sentence2) == ({'We', 'to', 'heavy', 'The', 'storm', 'meet', 'hit', \
    'pleased', 'are', 'by', 'a', 'in', 'was', 'you', 'our'}, {'really', 'city'})
```

<details>
	<summary>Solution</summary>

```python
def solution(sentence1, sentence2):
    set1 = set(sentence1.split())
    set2 = set(sentence2.split())

    return set1^set2, set1&set2
```
</details>

## 10. Prime Numbers Array

Given `k` numbers which are less than `n`, return the set of prime number among them

Note: The task is to write a program to print all Prime numbers in an Interval.

Definition: A prime number is a natural number greater than 1 that has no positive divisors other than 1 and itself.

```python
assert solution(35) == [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]
```

<details>
	<summary>Solution</summary>

```python
def solution(n):
    primes = []
    for num in range(2, n):
        for p in primes:
            if num % p == 0:
                break
        else:
            primes.append(num)
    return primes
```

</details>
