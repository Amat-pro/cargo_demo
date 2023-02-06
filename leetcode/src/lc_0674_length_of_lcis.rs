// 0647 最长连续递增子序列
fn length_of_lcis(nums: Vec<i32>) -> i32 {
    let mut result = 0;

    // 定义dp  nums[1]初始化为1
    let mut dp = vec![1; nums.len()];

    // 遍历
    for i in 1..nums.len() {
        if nums[i] > nums[i - 1] {
            dp[i] = dp[i - 1] + 1;
        }
        result = result.max(dp[i]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lcis() {
        assert_eq!(length_of_lcis(vec![1, 3, 5, 4, 7]), 3);
        assert_eq!(length_of_lcis(vec![2, 2, 2, 2, 2, 2]), 1)
    }
}
