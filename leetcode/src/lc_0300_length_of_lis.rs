// 0300 最长递增子序列
fn length_of_lis(nums: Vec<i32>) -> i32 {
    // 定义dp 初始化为1
    let mut dp = vec![1; nums.len()];

    let mut result = 1;

    // 遍历
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[j] < nums[i] {
                dp[i] = dp[i].max(dp[j] + 1);

                result = result.max(dp[i]);
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_length_of_lis() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
        assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);
        assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    }
}
