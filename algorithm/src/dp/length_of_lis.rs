/**
## 给你一个整数数组 nums ，找到其中最长严格递增子序列的长度
## 提示
* 1 <= nums.length <= 2500
* -10^4 nums\[i] <= 10^4
## Solution
* DP (n^2)

定义dp\[i]为nums\[i]结尾的最长子序列的长度，dp\[i]初始化为1，题目转化为求dp\[i]的最大值
状态转移方程为：
```
dp[i]=max(dp[j])+1 (0<=j<i && nums[i]>nums[j])
```

// todo length_of_lis 贪心+二分查找 n*log(n)
* 贪心+二分查找 n*log(n)

// todo length_of_lis 树状数组
* 树状数组


## Demo
*
```
输入：nums = [10,9,2,5,3,7,101,18]
输出：4
解释：最长递增子序列是 [2,3,7,101]，因此长度为 4
```

*
```
输入：nums = [0,1,0,3,2,3]
输出：4
```

*
```
输入：nums = [7,7,7,7,7,7,7]
输出：1
```
 **/
pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    // 默认每个dp[i]的初始值为1
    let mut dp = vec![1; len];

    // 计算每个dp[i]的值
    for i in 1..len {
        for j in 0..i {
            if nums[i] > nums[j] {
                //
                dp[i] = dp[i].max(dp[j] + 1)
            }
        }
    }
    println!("===> {:?}", dp);
    *dp.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        let nums = vec![0, 1, 0, 3, 2, 3];
        assert_eq!(4, length_of_lis(nums));

        let nums2 = vec![0, 1, 5, 0, 4, 2, 3];
        assert_eq!(4, length_of_lis(nums2));

        let nums3 = vec![10, 11, 12, 0, 1, 2, 3, 4, 5];
        assert_eq!(6, length_of_lis(nums3));
    }
}
