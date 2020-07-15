## Rust编程练习

> https://leetcode-cn.com/

选取Leetcode相关题目。

### 两数之和

> https://leetcode-cn.com/problems/two-sum/

给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。

你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。

#### Python

```python
class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        for i in range(len(nums)):
            for j in range(i + 1, len(nums)):
                if nums[i] + nums[j] == target:
                    return [i, j]
```

#### Rust

```rust
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                if nums[i] + nums[j] == target{
                    return vec![i as i32, j as i32];
                }
            }
        }
        nums
    }
}
```

发现这个系列的题目有些rust写起来很难。。先做些简单题水一水。。

### 汉明距离

> https://leetcode-cn.com/problems/hamming-distance/

两个整数之间的[汉明距离](https://baike.baidu.com/item/汉明距离)指的是这两个数字对应二进制位不同的位置的数目。

给出两个整数 `x` 和 `y`，计算它们之间的汉明距离。

#### Python

```python
class Solution:
    def hammingDistance(self, x: int, y: int) -> int:
        return bin(x ^ y).count("1")
```

#### Rust

```rust
impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        format!("{:b}", x^y)
            .chars()
            .filter(|x| x== &'1')
            .count() as i32
    }
}
```

### 只出现一次的数字

> https://leetcode-cn.com/problems/single-number/submissions/

给定一个**非空**整数数组，除了某个元素只出现一次以外，其余每个元素均出现两次。找出那个只出现了一次的元素。

#### Python

```python
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        for i in range(1, len(nums)):
            nums[i] ^= nums[i - 1]
        return nums[len(nums)- 1]
```

#### Rust

```rust
impl Solution {
    pub fn single_number(mut nums: Vec<i32>) -> i32 {
        for i in 1..nums.len(){
            nums[i] ^= nums[i - 1];
        }
        nums[nums.len() - 1]
    }
}
```

### 多数元素

> https://leetcode-cn.com/problems/majority-element

给定一个大小为 n 的数组，找到其中的多数元素。多数元素是指在数组中出现次数大于 ⌊ n/2 ⌋ 的元素。

你可以假设数组是非空的，并且给定的数组总是存在多数元素。

#### C++

```python
class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int count = 0, ret = -1, n = nums.size();
        for(int i = 0; i < n; ++i){
            if(count == 0){
                count ++; ret = nums[i];
            } else{
                count += nums[i] == ret ? 1 : -1;
            }
        }
        return ret;
    }
};
```

#### Rust

```rust
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;let mut ret: i32 = -1;
        let n = nums.len();
        for i in 0..n{
            if count == 0{
                count = 1;
                ret = nums[i];
            } else{
                if nums[i] == ret{
                    count += 1;
                } else{
                    count -= 1;
                }
            }
        }
        return ret
    }
}
```

### 合并两个有序链表**

> https://leetcode-cn.com/problems/merge-two-sorted-lists/

将两个升序链表合并为一个新的 **升序** 链表并返回。新链表是通过拼接给定的两个链表的所有节点组成的。 

#### Python

```python
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def mergeTwoLists(self, l1: ListNode, l2: ListNode) -> ListNode:
        if l1 == None or l2 == None:
            if l1 != None:
                return l1
            else:
                return l2

        ret = ListNode()
        rel_ret = ret
        if(l1.val < l2.val):
            ret.val = l1.val
            l1 = l1.next
        else:
            ret.val = l2.val
            l2 = l2.next
        
        while l1 != None and l2 != None:
            ret.next = ListNode()
            ret = ret.next
            if(l1.val < l2.val):
                ret.val = l1.val
                l1 = l1.next
            else:
                ret.val = l2.val
                l2 = l2.next
        while l1 != None:
            ret.next = ListNode()
            ret = ret.next
            ret.val = l1.val
            l1 = l1.next
        while l2 != None:
            ret.next = ListNode()
            ret = ret.next
            ret.val = l2.val
            l2 = l2.next
        return rel_ret
```

#### Rust

rust的链表真复杂。慢慢感觉到rust为了安全性在编程上造就的巨大门槛，这无疑是牺牲。

> https://leetcode-cn.com/problems/merge-two-sorted-lists/solution/rust-fei-di-gui-zui-jian-dan-cao-zuo-mo-shi-pi-pei/
>
> 参考这个写法，模式匹配大法好。
>
> `as_mut()`：https://doc.rust-lang.org/std/option/enum.Option.html#method.as_mut
>
> > Converts from `&mut Option<T>` to `Option<&mut T>`.

```rust
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut ret = Some(Box::new(ListNode::new(0)));
        let mut p = &mut ret;
        loop{
            match (l1.as_mut(), l2.as_mut()){
                (None, None) => {break;},
                (Some(a), None) => {
                    p.as_mut().unwrap().next = l1.take();
                    break;
                },
                (None, Some(b)) => {
                    p.as_mut().unwrap().next = l2.take();
                    break;
                },
                (Some(a), Some(b)) => {
                    if a.val < b.val{
                        let next = a.next.take();
                        p.as_mut().unwrap().next = l1.take();
                        l1 = next;
                    } else{
                        let next = b.next.take();
                        p.as_mut().unwrap().next = l2.take();
                        l2 = next;
                    }
                    p = &mut p.as_mut().unwrap().next;
                }
            }
        }
        ret.unwrap().next
    }
}
```

### 移动零

> https://leetcode-cn.com/problems/move-zeroes

给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。

示例:

输入: [0,1,0,3,12]
输出: [1,3,12,0,0]

说明:

> 必须在原数组上操作，不能拷贝额外的数组。
> 尽量减少操作次数。

#### Python

```python
class Solution:
    def moveZeroes(self, nums: List[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        l, r = 0, 0
        while l < len(nums):
            if nums[l] != 0:
                if nums[r] == 0:
                    nums[r] = nums[l]
                    nums[l] = 0
                    l += 1; r += 1
                    while nums[r] != 0:
                        r += 1
                else:
                    l += 1
                    r += 1
            else:
                l += 1
```

#### Rust

```rust
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let (mut l, mut r): (usize, usize) = (0, 0);
        while l < nums.len(){
            if nums[l] != 0{
                if nums[r] == 0{
                    nums[r] = nums[l];
                    nums[l] = 0;
                    l += 1; r += 1;
                    while nums[r] != 0 { r += 1;}
                } else{
                    l += 1;
                    r += 1;
                }
            } else{
                l += 1;
            }
        }
    }
}
```

### [找到所有数组中消失的数字](https://leetcode-cn.com/problems/find-all-numbers-disappeared-in-an-array/)

> https://leetcode-cn.com/problems/find-all-numbers-disappeared-in-an-array/

给定一个范围在  1 ≤ a[i] ≤ n ( n = 数组大小 ) 的 整型数组，数组中的元素一些出现了两次，另一些只出现一次。

找到所有在 [1, n] 范围之间没有出现在数组中的数字。

您能在不使用额外空间且时间复杂度为O(n)的情况下完成这个任务吗? 你可以假定返回的数组不算在额外空间内。

示例:

输入:
[4,3,2,7,8,2,3,1]

输出:
[5,6]

#### Python

```python
class Solution:
    def findDisappearedNumbers(self, nums: List[int]) -> List[int]:
        for i in range(0, len(nums)):
            if nums[abs(nums[i]) - 1] > 0:
                nums[abs(nums[i]) - 1] *= -1
        print(nums)
        ret = []
        for i  in range(0, len(nums)):
            if nums[i] > 0:
                ret.append(i + 1)
        return ret
```

#### Rust

```rust
impl Solution {
    pub fn find_disappeared_numbers(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len(){
            let index = nums[i].abs() as usize - 1;
            if nums[index] > 0{
                nums[index] *= -1;
            }
        }
        println!("{:?}", nums);
        let mut ret = Vec::<i32>::new();
        for i in 0..nums.len(){
            if nums[i] > 0{
                ret.push((i + 1) as i32);
            }
        }
        ret
    }
}
```

### 动态和

> https://leetcode-cn.com/problems/running-sum-of-1d-array

给你一个数组 nums 。数组「动态和」的计算公式为：runningSum[i] = sum(nums[0]…nums[i]) 。

请返回 nums 的动态和。

#### Python

```python
class Solution:
    def runningSum(self, nums: List[int]) -> List[int]:
        for i in range(1, len(nums)):
            nums[i] += nums[i - 1]
        return nums
```

#### Rust

```rust
impl Solution {
    pub fn running_sum(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 1..nums.len(){
            nums[i] += nums[i - 1];
        }
        nums
    }
}
```

> 又水了一道题~

### 好数对的数目

> https://leetcode-cn.com/problems/number-of-good-pairs

给你一个整数数组 nums 。

如果一组数字 (i,j) 满足 nums[i] == nums[j] 且 i < j ，就可以认为这是一组 好数对 。

返回好数对的数目。

#### Python

```python
class Solution:
    def numIdenticalPairs(self, nums: List[int]) -> int:
        ret = 0
        for i in range(0, len(nums)):
            for j in range(i+1, len(nums)):
                if nums[i] == nums[j]:
                    ret += 1
        return ret
```

#### Rust

```rust
impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut ret = 0;
        for i in 0..nums.len(){
            for j in (i+1)..nums.len(){
                if nums[i] == nums[j]{
                    ret += 1;
                }
            }
        }
        ret
    }
}
```

### [重新排列数组](https://leetcode-cn.com/problems/shuffle-the-array/)

> https://leetcode-cn.com/problems/shuffle-the-array

给你一个数组 nums ，数组中有 2n 个元素，按 [x1,x2,...,xn,y1,y2,...,yn] 的格式排列。

请你将数组按 [x1,y1,x2,y2,...,xn,yn] 格式重新排列，返回重排后的数组。

示例 1：

输入：nums = [2,5,1,3,4,7], n = 3
输出：[2,3,5,4,1,7] 
解释：由于 x1=2, x2=5, x3=1, y1=3, y2=4, y3=7 ，所以答案为 [2,3,5,4,1,7]
示例 2：

输入：nums = [1,2,3,4,4,3,2,1], n = 4
输出：[1,4,2,3,3,2,4,1]
示例 3：

输入：nums = [1,1,2,2], n = 2
输出：[1,2,1,2]

#### Python

```python
class Solution:
    def shuffle(self, nums: List[int], n: int) -> List[int]:
        ret = [];
        for i in range(0,n):
            ret.append(nums[i]);
            ret.append(nums[(n + i)]);
        return ret
```

#### Rust

```rust
impl Solution {
    pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::<i32>::new();
        for i in 0..n{
            ret.push(nums[i as usize]);
            ret.push(nums[(n + i) as usize]);
        }
        ret
    }
}
```

### [回文数](https://leetcode-cn.com/problems/palindrome-number/)

> https://leetcode-cn.com/problems/palindrome-number/

判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

#### Python

```python
class Solution:
    def isPalindrome(self, x: int) -> bool:
        s = str(x);
        i, n = 0, len(s) - 1;
        while (i < n - i):
            if (s[i] != s[n - i]):
                return False
            i += 1
        return True
```

#### Rust

```rust
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s1 = x.to_string();
        let s2 = x.to_string().chars().rev().collect::<String>();
        let ret:bool = (s1 == s2);
        ret
    }
}
```

### [整数反转](https://leetcode-cn.com/problems/reverse-integer/)

> https://leetcode-cn.com/problems/reverse-integer/

给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

#### Python

```python
class Solution:
    def reverse(self, x: int) -> int:
        if x < 0:
            ret =  -int(str(-int(x))[::-1])
        else:
            ret =  int(str(int(x))[::-1])
        if ret < -pow(2,31) or ret > pow(2, 31) - 1:
            return 0
        else:
            return ret
```

#### Rust

```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut ret: i64 = 0;
        let x = x as i64;
        if x < 0{
            ret =  - (-x).to_string()
                        .chars()
                        .rev()
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap();
            println!("{}", ret);
        } else{
            ret =       x.to_string()
                        .chars()
                        .rev()
                        .collect::<String>()
                        .parse::<i64>()
                        .unwrap();
            println!("{}", ret);
        }
        if (ret as i64) < -2i64.pow(31) || (ret as i64) > (2i64.pow(31) - 1){
            return 0i32
        }
        ret as i32
    }
}
```

### [最大子序和](https://leetcode-cn.com/problems/maximum-subarray/)

> https://leetcode-cn.com/problems/maximum-subarray/

给定一个整数数组 `nums` ，找到一个具有最大和的连续子数组（子数组最少包含一个元素），返回其最大和。

#### Python

```python
class Solution:
    def maxSubArray(self, nums: List[int]) -> int:
        best, current = nums[0], nums[0]
        for i in range(1, len(nums)):
            if current < 0:
                current = 0
            current += nums[i]
            if current > best:
                best = current
        return best
```

#### Rust

```rust
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut best: i32    = nums[0];
        let mut current: i32 = nums[0];
        for i in 1..nums.len(){
            if current < 0{
                current = 0;
            }
            current += nums[i];
            if current > best{
                best = current;
            }
        }
        best
    }
}
```

### [爬楼梯](https://leetcode-cn.com/problems/climbing-stairs/)

> https://leetcode-cn.com/problems/climbing-stairs/

假设你正在爬楼梯。需要 n 阶你才能到达楼顶。

每次你可以爬 1 或 2 个台阶。你有多少种不同的方法可以爬到楼顶呢？

注意：给定 n 是一个正整数。

示例 1：

输入： 2
输出： 2
解释： 有两种方法可以爬到楼顶。

1.  1 阶 + 1 阶
2.  2 阶
    示例 2：

输入： 3
输出： 3
解释： 有三种方法可以爬到楼顶。

1.  1 阶 + 1 阶 + 1 阶
2.  1 阶 + 2 阶
3.  2 阶 + 1 阶

#### Python

```python
class Solution:
    def climbStairs(self, n: int) -> int:
        f = [1] * (n + 1)
        for i in range(2, n + 1):
            f[i] = f[i - 1] + f[i - 2]
        return f[n]
```

#### Rust

```rust
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        let mut f = vec![1 as i32; (n + 1) as usize];
        for i in 2..(n+1){
            f[(i) as usize] = f[(i - 1) as usize] + f[(i - 2) as usize];
        }
        f[n as usize]
    }
}
```

### [杨辉三角](https://leetcode-cn.com/problems/pascals-triangle/)

> https://leetcode-cn.com/problems/pascals-triangle/

给定一个非负整数 *numRows，*生成杨辉三角的前 *numRows* 行。

![img](README/PascalTriangleAnimated2.gif)

在杨辉三角中，每个数是它左上方和右上方的数的和。

#### Rust

用组合数的思路做了做，发现想多了。。。TLE！

```rust
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
        fn func(n: i32, m: i32) -> i32{
            if m == 0 || m == n{
                return 1;
            }
            func(n -1, m) + func(n - 1, m - 1)
        }
        for i in 0..num_rows{
            let mut t : Vec<i32> = Vec::<i32>::new();
            for j in 0..=i{
                t.push(func(i as i32, j as i32));
            }
            ret.push(t);
            println!("{:?}", ret);
        }
        ret
    }
}
```

接下来用杨辉三角生成的方法：

```rust
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
        for i in 0..num_rows{
            let mut t : Vec<i32> = vec![1 as i32 ; (i + 1) as usize];
            ret.push(t);
            println!("{:?}", ret);
        }
        for i in 0..num_rows{
            for j in 1..i{
                ret[i as usize][j as usize] = ret[(i-1) as usize][(j-1) as usize] + ret[(i-1) as usize][j as usize];
            }
            println!("{:?}", ret);
        }
        ret
    }
}
```

#### Python

```python
class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        ret = []
        for i in range(0, numRows):
            ret.append([1] * (i + 1))
        print(ret)
        for i in range(0, numRows):
            for j in range(1, i):
                ret[i][j] = ret[i - 1][j - 1] + ret[i - 1][j]
        return ret
```