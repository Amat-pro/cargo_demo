// 0053 最大子序列和

fn max_sub_array(nums: Vec<i32>) -> i32 {
    // 定义dp
    let mut dp = vec![0; nums.len()];
    // 初始化dp
    dp[0] = nums[0];

    let mut result = dp[0];
    // 遍历
    for i in 1..nums.len() {
        dp[i] = (dp[i - 1] + nums[i]).max(nums[i]);

        result = result.max(dp[i])
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sub_array() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
    }
}
