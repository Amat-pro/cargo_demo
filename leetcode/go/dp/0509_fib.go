package dp

func fib(n int) int {
	if n < 0 {
		return n
	}

	if n == 0 {
		return 0
	}

	if n == 1 {
		return 1
	}

	// 定义
	dp := make([]int, n+1)
	// 初始化
	dp[0] = 0
	dp[1] = 1
	// 遍历  （从前向后）
	for i := 2; i < n+1; i++ {
		// 递推公式
		dp[i] = dp[i-1] + dp[i-2]
	}

	return dp[n]
}
