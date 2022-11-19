/**
# DP-无重叠区间
# 描述
给定一个区间的集合 intervals ，其中 intervals\[i] = \[starti, endi] 返回需要移除区间的最小数量，使剩余区间互不重叠
# Demo
*
```
输入: intervals = [[1,2],[2,3],[3,4],[1,3]]
输出: 1
解释: 移除 [1,3] 后，剩下的区间没有重叠
```

*
```
输入: intervals = [ [1,2], [1,2], [1,2] ]
输出: 2
解释: 你需要移除两个 [1,2] 来使剩下的区间没有重叠
```

*
```
输入: intervals = [ [1,2], [2,3] ]
输出: 0
解释: 你不需要移除任何区间，因为它们已经是无重叠的了
```

# 提示
```
1. 1 <= intervals.length <= 105
2. intervals[i].length == 2
3. -5 * 104 <= starti < endi <= 5 * 104
```

 **/

pub fn erase_overlap_intervals(_intervals: Vec<Vec<i32>>) -> i32 {
    0
}