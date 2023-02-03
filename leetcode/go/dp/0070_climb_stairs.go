package dp

func climbStairs(n int) int {
	if n == 1 {
		return 1
	}

	// 定义dp
	dp := make([]int, n+1)
	// 初始化
	dp[1] = 1
	dp[2] = 2

	// 遍历
	for i := 3; i < n+1; i++ {
		dp[i] = dp[i-1] + dp[i-2]
	}

	return dp[n]

}
