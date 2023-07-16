// 0718 最长重复子数组

fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
    let mut result = 0;

    // 定义dp
    // 初始化为0
    let mut dp: Vec<Vec<i32>> = vec![vec![0; nums2.len() + 1]; nums1.len() + 1];
    // 遍历
    for i in 1..nums1.len() + 1 {
        for j in 1..nums2.len() + 1 {
            // 递推公式
            if nums1[i - 1] == nums2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            }
            result = result.max(dp[i][j])
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_length() {
        assert_eq!(find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]), 3);
        assert_eq!(find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]), 5);
    }
}
