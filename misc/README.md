# Problems

## 01

Given the heads of two sorted linked lists, list1 and list2, merge those lists into one sorted list and return the head of this list.

## 02

Implement a datastructure called TimeMap with two operations: `get(key, timestamp)`, `set(key, value, timestamp)`. By using set you can set the value of a specific key at a timestamp. With get you can retrieve the value of a key at a given timestamp. get should always return the value of the key at a given timestamp. Meaning that when calling `set("foo", "low", 1)` and `set("foo", "high", 5)` , `get("foo", 3)` should return "low" as this is the value of the key at that timestamp.

## 03

Implement a datastructure which supports calculating the median of a stream of integers. The datastructure should support two operations `addNum(num)` and `findMedian()`. `addNum` adds another number to your data, while `findMedian` should find the median of all numbers that have been added so far.