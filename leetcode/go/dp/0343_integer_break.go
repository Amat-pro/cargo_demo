package dp

// 拆分整数
func integerBreak(n int) int {
	// 定义dp
	dp := make([]int, n+1)

	// 初始化dp
	dp[0] = 0
	dp[1] = 0
	dp[2] = 1

	// 遍历
	for i := 3; i <= n; i++ {
		// 固定j
		for j := 1; j < i; j++ {
			dp[i] = max(dp[i], max(j*(i-j), j*dp[i-j]))
		}
	}

	return dp[n]

}

func max(a, b int) int {
	if a >= b {
		return a
	}

	return b
}
