package dp

func numTrees(n int) int {

	// 定义dp
	dp := make([]int, n+1)

	// 初始化
	dp[0] = 1
	dp[1] = 1

	// 遍历
	for i := 2; i <= n; i++ {
		// 确定头结点
		for j := 1; j <= i; j++ {
			// 递推公式
			dp[i] += dp[j-1] * dp[i-j]
		}
	}

	return dp[n]
}
