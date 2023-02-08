// 1035 不相交的线

// 等价于求两个数组最长公共子序列长度
fn max_uncrossed_lines(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    // 定义dp 并 初始化
    let mut dp = vec![vec![0; nums2.len()]; nums1.len()];

    // 遍历
    for i in 1..=nums1.len() {
        for j in 1..=nums2.len() {
            // 递推公式
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    dp[nums1.len()][nums2.len()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_uncrossed_lines() {
        assert_eq!(max_uncrossed_lines(vec![1, 4, 2], vec![1, 2, 4]), 2);
        assert_eq!(
            max_uncrossed_lines(vec![2, 5, 1, 2, 5], vec![10, 5, 2, 1, 5, 2]),
            3
        );
        assert_eq!(
            max_uncrossed_lines(vec![1, 3, 7, 1, 7, 5], vec![1, 9, 2, 5, 1]),
            2
        );
    }
}
