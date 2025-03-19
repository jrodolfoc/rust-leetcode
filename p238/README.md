# 238. Product of Array Except Self

Given an integer array `nums`, return _an array_ `answer` _such that_ `answer[i]` _is equal to the product of all the elements of_ `nums` _except_ `nums[i]`.

The product of any prefix or suffix of `nums` is __guaranteed__ to fit in a __32-bit__ integer.

You must write an algorithm that runs in `O(n)` time and without using the division operation.

### Example 1:

**Input:** nums = [1,2,3,4]

**Output:** [24,12,8,6]

### Example 2:

**Input:** nums = [-1,1,0,-3,3]

**Output:** [0,0,9,0,0]
 
### Constraints:

- `2 <= nums.length <= 10^5`

- `-30 <= nums[i] <= 30`

- The input is generated such that `answer[i]` is __guaranteed__ to fit in a __32-bit__ integer.


__Follow up:__ Can you solve the problem in `O(1)` extra space complexity? (The output array __does not__ count as extra space for space complexity analysis.)

### URL

https://leetcode.com/problems/product-of-array-except-self/description/?envType=study-plan-v2&envId=top-interview-150